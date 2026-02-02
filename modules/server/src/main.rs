use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{App, HttpServer, web};
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
  jwt::JwtKeys,
  logging::init_logging,
};
use presentation::{http, middleware::jwt::JwtAuthMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  init_logging();

  let config = AppConfig::from_env().expect("Failed reading env variables");
  let pool = create_pool(&config.database_url)
    .await
    .expect("Failed to connect to database");

  run_migrations(&pool)
    .await
    .expect("Failed to run migrations");

  let posts_repo = Arc::new(PostgresPostRepository::new(pool.clone()));
  let users_repo = Arc::new(PostgresUserRepository::new(pool.clone()));

  let blog_service = BlogService::new(Arc::clone(&posts_repo));
  let auth_service = AuthService::new(
    Arc::clone(&users_repo),
    JwtKeys::new(config.jwt_secret.clone()),
  );
  let jwt_keys = auth_service.keys().clone();
  let config_clone = config.clone();

  HttpServer::new(move || {
    let cors = build_cors(&config_clone);
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
      .service(
        web::scope("/api")
          .configure(http::auth::configure)
          .configure(http::posts::configure_public)
          .service(
            web::scope("")
              .wrap(JwtAuthMiddleware::new(jwt_keys.clone()))
              .configure(http::posts::configure_protected),
          ),
      )
  })
  .bind((config.host.as_str(), config.port))?
  .run()
  .await
}
