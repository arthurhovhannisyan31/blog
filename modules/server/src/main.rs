use actix_web::{
  middleware::{DefaultHeaders, Logger}, web,
  App,
  HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use std::sync::Arc;

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

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
  middleware::jwt::jwt_validator,
};

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

  let jwt_service = JwtService::new(config.jwt_secret.clone());
  let jwt_service_clone = jwt_service.clone();

  let posts_repo = Arc::new(PostgresPostRepository::new(pool.clone()));
  let users_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
  let blog_service = BlogService::new(Arc::clone(&posts_repo));
  let auth_service = AuthService::new(Arc::clone(&users_repo), jwt_service);

  HttpServer::new(move || {
    let cors = build_cors(&config_data);
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
      .app_data(web::Data::new(blog_service.clone()))
      .app_data(web::Data::new(auth_service.clone()))
      .app_data(web::Data::new(jwt_service_clone.clone()))
      .service(
        web::scope("/api")
          .service(public_scope())
          .service(web::scope("").wrap(auth).service(protected_scope())),
      )
  })
  .bind((config.host.as_str(), config.port))?
  .run()
  .await
}
