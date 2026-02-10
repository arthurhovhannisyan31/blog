use std::net::SocketAddr;

use blog_client::{Transport, client::BlogClient};

use crate::configs::AppConfig;
use crate::error::CliError;

pub async fn init_client(
  grpc: bool,
  server: Option<SocketAddr>,
) -> Result<BlogClient, CliError> {
  let AppConfig {
    host,
    grpc_port,
    http_port,
  } = AppConfig::from_env()?;

  let transport: Transport;
  let http_addr = format!("http://{host}:{http_port}/api");
  let grpc_addr = format!("http://{host}:{grpc_port}");

  if let Some(server_addr) = server {
    transport = Transport::Http(server_addr.to_string());
  } else {
    if grpc {
      transport = Transport::Grpc(grpc_addr);
    } else {
      transport = Transport::Http(http_addr);
    }
  }

  BlogClient::new(transport).await.map_err(CliError::from)
}
