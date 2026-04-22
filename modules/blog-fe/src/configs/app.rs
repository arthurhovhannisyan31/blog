pub struct AppConfig {
  pub port: String,
  pub host: String,
}

impl AppConfig {
  pub fn from_env() -> anyhow::Result<Self> {
    let port = std::env::var("API_PORT").unwrap_or_else(|_| "8080".into());
    let host = std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".into());

    Ok(AppConfig { port, host })
  }
}
