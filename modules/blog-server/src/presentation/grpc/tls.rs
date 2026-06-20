#[cfg(feature = "tls")]
use {
  crate::infrastructure::config::AppConfig,
  crate::infrastructure::error::ServerError,
  anyhow::Context,
  std::fs,
  tonic::transport::{Identity, ServerTlsConfig},
};

#[cfg(feature = "tls")]
pub fn build_grpc_tls_config(
  app_config: &AppConfig,
) -> Result<ServerTlsConfig, ServerError> {
  let cert = fs::read_to_string(&app_config.tls_crt_path)
    .context("Failed to read TLS certificate")?;
  let key = fs::read_to_string(&app_config.tls_key_path)
    .context("Failed to read TLS certificate key")?;
  let identity = Identity::from_pem(cert, key);

  Ok(ServerTlsConfig::new().identity(identity))
}
