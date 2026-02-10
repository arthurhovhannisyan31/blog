use serde::Deserialize;

use crate::infrastructure::error::ServerError;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
  pub host: String,
  pub http_port: u16,
  pub grpc_port: u16,
  pub database_url: String,
  pub jwt_secret: String,
  #[serde(default)]
  pub cors_origins: Vec<String>,
}

impl AppConfig {
  pub fn from_env() -> Result<Self, ServerError> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());

    let http_port = std::env::var("HTTP_PORT")
      .unwrap_or_else(|_| "8080".into())
      .parse()
      .map_err(|e| {
        ServerError::VarError(format!("Invalid PORT variable: {e}"))
      })?;
    let grpc_port = std::env::var("GRPC_PORT")
      .unwrap_or_else(|_| "50051".into())
      .parse()?;
    let database_url = std::env::var("DATABASE_URL").map_err(|e| {
      ServerError::VarError(format!("Missing DATABASE_URL: {e}"))
    })?;
    let jwt_secret = std::env::var("JWT_SECRET")
      .map_err(|e| ServerError::VarError(format!("Missing JWT_SECRET: {e}")))?;

    let cors_origins = std::env::var("CORS_ORIGINS")
      .unwrap_or_else(|_| "*".into())
      .split(',')
      .map(|s| s.trim().to_string())
      .filter(|s| !s.is_empty())
      .collect();

    Ok(Self {
      host,
      http_port,
      grpc_port,
      database_url,
      jwt_secret,
      cors_origins,
    })
  }
}
