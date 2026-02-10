use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;

use crate::infrastructure::error::ServerError;

pub async fn create_pool(database_url: &str) -> Result<PgPool, ServerError> {
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .min_connections(2)
    .acquire_timeout(std::time::Duration::from_secs(5))
    .connect(database_url)
    .await
    .map_err(|e| {
      ServerError::SqlxError(format!("Failed connecting to Postgres: {e}"))
    })?;

  info!("connected to PostgreSQL");
  Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), ServerError> {
  info!("running database migrations");
  sqlx::migrate!().run(pool).await.map_err(|e| {
    ServerError::SqlxError(format!("Failed running migration: {e}"))
  })?;

  info!("migrations completed");
  Ok(())
}
