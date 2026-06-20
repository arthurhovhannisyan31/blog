use std::env;

pub struct AppConfig {
  pub port: String,
  pub host: String,
  pub protocol: String,
}

impl AppConfig {
  pub fn from_env() -> anyhow::Result<Self> {
    let host = env::var("FRONTEND_API_HOST").unwrap_or(
      option_env!("FRONTEND_API_HOST")
        .unwrap_or("127.0.0.1")
        .into(),
    );
    let port = env::var("FRONTEND_API_PORT")
      .unwrap_or(option_env!("FRONTEND_API_PORT").unwrap_or("8080").into());
    let use_secure_connection = env::var("BACKEND_TLS")
      .unwrap_or(option_env!("BACKEND_TLS").unwrap_or("false").into())
      .eq("true");

    let protocol = (if use_secure_connection {
      "https"
    } else {
      "http"
    })
    .into();

    Ok(AppConfig {
      port,
      host,
      protocol,
    })
  }
}
