use std::env;

pub struct AppConfig {
  pub port: String,
  pub host: String,
}

impl AppConfig {
  pub fn from_env() -> anyhow::Result<Self> {
    let host = env::var("FRONTEND_API_HOST").unwrap_or(
      option_env!("FRONTEND_API_HOST")
        .unwrap_or("localhost")
        .into(),
    );
    let port = env::var("FRONTEND_API_PORT")
      .unwrap_or(option_env!("FRONTEND_API_PORT").unwrap_or("3001").into());

    Ok(AppConfig { port, host })
  }
}
