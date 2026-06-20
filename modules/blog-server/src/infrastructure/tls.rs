use crate::infrastructure::error::ServerError;

#[cfg(feature = "tls")]
pub fn init_tls() -> Result<(), ServerError> {
  rustls::crypto::ring::default_provider()
    .install_default()
    .map_err(|err| {
      anyhow::anyhow!("Failed to install rustls crypto provider: {:?}", err)
    })?;

  Ok(())
}
