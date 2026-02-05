use std::net::SocketAddr;
use std::sync::Arc;

use actix_web::{
  App, HttpServer,
  middleware::{DefaultHeaders, Logger},
  web,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use tokio::select;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use crate::presentation::grpc::server::GrpcBlogServiceImpl;
use application::{auth_service::AuthService, blog_service::BlogService};
use data::{
  post_repository::PostgresPostRepository,
  user_repository::PostgresUserRepository,
};
use infrastructure::{
  config::AppConfig,
  cors::build_cors,
  database::{create_pool, run_migrations},
  jwt::JwtService,
  logging::init_logging,
};
use presentation::{
  http::scoped::{protected_scope, public_scope},
  middleware::jwt_validator,
};
use proto_generator::blog::FILE_DESCRIPTOR;
use proto_generator::blog::grpc_blog_service_server::GrpcBlogServiceServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  init_logging();

  let config = AppConfig::from_env().expect("Failed reading env variables");
  let config_data = config.clone();
  let pool = create_pool(&config.database_url)
    .await
    .expect("Failed to connect to database");

  run_migrations(&pool)
    .await
    .expect("Failed to run migrations");

  let jwt_service = Arc::new(JwtService::new(config.jwt_secret.clone()));

  let posts_repo = PostgresPostRepository::new(pool.clone());
  let blog_service = BlogService::new(posts_repo);

  let users_repo = PostgresUserRepository::new(pool.clone());
  let auth_service = AuthService::new(users_repo, jwt_service.clone());

  let jwt_service_clone = jwt_service.clone();
  let auth_service_clone = auth_service.clone();
  let blog_service_clone = blog_service.clone();

  // actix server init
  let http_server = HttpServer::new(move || {
    let cors = build_cors(&config_data.cors_origins);
    let auth = HttpAuthentication::with_fn(jwt_validator);

    App::new()
      .wrap(Logger::default())
      .wrap(
        DefaultHeaders::new()
          .add(("X-Content-Type-Options", "nosniff"))
          .add(("Referrer-Policy", "no-referrer"))
          .add(("Permissions-Policy", "geolocation=()"))
          .add(("Cross-Origin-Opener-Policy", "same-origin")),
      )
      .wrap(cors)
      .app_data(web::Data::new(blog_service_clone.clone()))
      .app_data(web::Data::new(auth_service_clone.clone()))
      .app_data(web::Data::new(jwt_service_clone.clone()))
      .service(
        web::scope("/api")
          .service(public_scope())
          .service(web::scope("").wrap(auth).service(protected_scope())),
      )
  })
  .bind((config.host.as_str(), config.http_port))?
  .run();

  let jwt_service_clone = jwt_service.clone();
  let auth_service_clone = auth_service.clone();
  let blog_service_clone = blog_service.clone();

  // tonic server init
  let grpc_service = GrpcBlogServiceImpl::new(
    auth_service_clone,
    blog_service_clone,
    jwt_service_clone,
  );

  let grpc_reflection_service = Builder::configure()
    .register_encoded_file_descriptor_set(FILE_DESCRIPTOR)
    .build_v1()
    .expect("Failed building grpc reflection service");

  let grpc_addr_str = format!("{}:{}", config.host.as_str(), config.grpc_port);
  let grpc_addr: SocketAddr = grpc_addr_str
    .parse()
    .expect("Failed parsing grpc socket address");

  let grpc_server = Server::builder()
    .add_service(grpc_reflection_service)
    .add_service(GrpcBlogServiceServer::new(grpc_service))
    .serve(grpc_addr);

  select! {
      http_res = http_server => {
          if let Err(e) = http_res {
              eprintln!("Actix server error: {}", e);
          }
      },
      grpc_res = grpc_server => {
          if let Err(e) = grpc_res {
              eprintln!("Tonic server error: {}", e);
          }
      },
  };
  Ok(())
}
