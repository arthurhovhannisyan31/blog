use std::sync::Arc;

use tokio::select;

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
  database::{create_pool, run_migrations},
  error::ServerError,
  jwt::JwtService,
  logging::init_logging,
};
use presentation::{
  grpc::init::init_grpc_server, http::init::init_http_server,
};

#[actix_web::main]
async fn main() -> Result<(), ServerError> {
  init_logging();

  let config = AppConfig::from_env()?;
  let pool = create_pool(&config.database_url).await?;

  run_migrations(&pool).await?;

  let jwt_service = Arc::new(JwtService::new(config.jwt_secret.clone()));

  let posts_repo = PostgresPostRepository::new(pool.clone());
  let blog_service = BlogService::new(posts_repo);

  let users_repo = PostgresUserRepository::new(pool.clone());
  let auth_service = AuthService::new(users_repo, jwt_service.clone());

  let http_server = init_http_server(
    auth_service.clone(),
    blog_service.clone(),
    jwt_service.clone(),
    config.clone(),
  )?;

  let grpc_server = init_grpc_server(
    auth_service.clone(),
    blog_service.clone(),
    jwt_service.clone(),
    &config,
  );

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
