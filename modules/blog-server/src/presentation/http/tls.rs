#[cfg(feature = "tls")]
use {
  crate::infrastructure::config::AppConfig,
  crate::infrastructure::error::ServerError,
  anyhow::Context,
  rustls::ServerConfig,
  rustls_pki_types::{CertificateDer, PrivateKeyDer},
  std::fs::File,
  std::io::BufReader,
};

#[cfg(feature = "tls")]
pub fn build_http_tls_config(
  config: &AppConfig,
) -> Result<ServerConfig, ServerError> {
  let (cert, key) = load_certs_and_key(&config)?;

  let mut config = ServerConfig::builder()
    .with_no_client_auth()
    .with_single_cert(cert, key)
    .unwrap();

  config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
  Ok(config)
}

#[cfg(feature = "tls")]
fn load_certs_and_key(
  config: &AppConfig,
) -> Result<(Vec<CertificateDer<'static>>, PrivateKeyDer<'static>), ServerError>
{
  let cert_file =
    File::open(&config.tls_crt_path).context("Failed to open cert file")?;
  let certs: Vec<CertificateDer<'static>> =
    rustls_pemfile::certs(&mut BufReader::new(cert_file))
      .collect::<Result<Vec<_>, _>>()
      .context("Failed to parse certificates")?;

  let key_file =
    File::open(&config.tls_key_path).expect("Failed to open key file");
  let key = rustls_pemfile::private_key(&mut BufReader::new(key_file))
    .context("Failed to parse private key")?
    .context("No private key found in file")?;

  Ok((certs, key))
}
