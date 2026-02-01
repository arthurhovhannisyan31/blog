mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use infrastructure::{
  config::AppConfig,
  database::{create_pool, run_migrations},
  logging::init_logging,
};

#[actix_web::main]
async fn main() {
  init_logging();

  let config = AppConfig::from_env().expect("Failed reading env variables");
  let pool = create_pool(&config.database_url)
    .await
    .expect("Failed to connect to database");
  run_migrations(&pool)
    .await
    .expect("Failed to run migrations");
}
