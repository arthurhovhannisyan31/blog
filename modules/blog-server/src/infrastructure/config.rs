use crate::infrastructure::error::ServerError;
use anyhow::Context;
use serde::Deserialize;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
  pub host: String,
  pub http_port: u16,
  pub grpc_addr: SocketAddr,
  pub database_url: String,
  pub jwt_secret: String,
  #[serde(default)]
  pub cors_origins: Vec<String>,
  pub tls_key_path: PathBuf,
  pub tls_crt_path: PathBuf,
}

impl AppConfig {
  pub fn from_env() -> Result<Self, ServerError> {
    let docker_container =
      env::var("DOCKER_CONTAINER").unwrap_or("false".to_owned());
    // Load variables when run locally
    if &docker_container == "false" {
      dotenvy::dotenv()?;
    }

    let host = env::var("BACKEND_HOST").unwrap_or_else(|_| "localhost".into());

    let http_port = env::var("BACKEND_HTTP_PORT")
      .unwrap_or_else(|_| "8080".into())
      .parse()
      .map_err(|e| {
        ServerError::VarError(format!(
          "Invalid BACKEND_HTTP_PORT variable: {e}"
        ))
      })?;
    let grpc_port = env::var("BACKEND_GRPC_PORT")
      .unwrap_or_else(|_| "50051".into())
      .parse::<u16>()?;
    let grpc_addr: SocketAddr = format!("{}:{}", host.as_str(), grpc_port)
      .parse()
      .context("Failed parsing gRPC socket address")?;

    let database_url = env::var("BACKEND_DATABASE_URL").map_err(|e| {
      ServerError::VarError(format!("Missing BACKEND_DATABASE_URL: {e}"))
    })?;
    let jwt_secret = env::var("BACKEND_JWT_SECRET").map_err(|e| {
      ServerError::VarError(format!("Missing BACKEND_JWT_SECRET: {e}"))
    })?;

    let cors_origins = env::var("BACKEND_CORS_ORIGINS")
      .unwrap_or_else(|_| "*".into())
      .split(',')
      .map(|s| s.trim().to_string())
      .filter(|s| !s.is_empty())
      .collect();

    #[allow(unused_assignments)]
    let mut tls_key_path = PathBuf::default();
    #[allow(unused_assignments)]
    let mut tls_crt_path = PathBuf::default();

    #[cfg(feature = "tls")]
    {
      use anyhow::Context;

      let tls_key = env::var("TLS_KEY").map_err(|e| {
        ServerError::VarError(format!("Missing TLS_KEY_PATH: {e}"))
      })?;
      tls_key_path = PathBuf::from(&tls_key);
      tls_key_path
        .try_exists()
        .context("The TLS key file is not found")?;

      let tls_crt = env::var("TLS_CRT").map_err(|e| {
        ServerError::VarError(format!("Missing TLS_CRT_PATH: {e}"))
      })?;
      tls_crt_path = PathBuf::from(&tls_crt);
      tls_crt_path
        .try_exists()
        .context("The TLS crt file is not found")?;
    }

    Ok(Self {
      host,
      http_port,
      grpc_addr,
      database_url,
      jwt_secret,
      cors_origins,
      tls_key_path,
      tls_crt_path,
    })
  }
}
