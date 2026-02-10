use std::net::SocketAddr;

use clap::builder::NonEmptyStringValueParser;
use clap::{Parser, Subcommand};

use crate::error::CliError;

#[derive(Debug)]
pub struct AppConfig {
  pub host: String,
  pub http_port: u16,
  pub grpc_port: u16,
}

impl AppConfig {
  pub fn from_env() -> Result<Self, CliError> {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or("127.0.0.1".into());
    let http_port = std::env::var("HTTP_PORT")
      .unwrap_or("8080".into())
      .parse()
      .map_err(|e| {
        CliError::VarError(format!("Invalid HTTP_PORT variable: {e}"))
      })?;
    let grpc_port = std::env::var("GRPC_PORT")
      .unwrap_or("50051".into())
      .parse()
      .map_err(|e| {
        CliError::VarError(format!("Invalid GRPC_PORT variable: {e}"))
      })?;

    Ok(Self {
      host,
      http_port,
      grpc_port,
    })
  }
}

#[derive(Debug, Parser)]
#[command(name = "blog-cli", about, version, next_line_help = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
  #[arg(short, long, value_name = "GRPC protocol")]
  pub grpc: bool,
  #[arg(short, long, value_name = "Server address", value_parser = socket_address_validation)]
  pub server: Option<SocketAddr>,
}

/* u64 is selected to avoid additional validation logic
for parsed values of 'id', 'limit', 'offset' fields */
#[derive(Debug, Subcommand)]
pub enum Commands {
  Register {
    #[arg(short, long, value_parser = NonEmptyStringValueParser::new())]
    username: String,
    #[arg(short, long, value_parser = NonEmptyStringValueParser::new())]
    email: String,
    #[arg(short, long, value_parser = NonEmptyStringValueParser::new())]
    password: String,
  },
  Login {
    #[arg(short, long, value_parser = NonEmptyStringValueParser::new())]
    email: String,
    #[arg(short, long, value_parser = NonEmptyStringValueParser::new())]
    password: String,
  },
  Create {
    #[arg(short, long, value_parser = NonEmptyStringValueParser::new())]
    title: String,
    #[arg(short, long)]
    content: String,
  },
  Get {
    #[arg(short, long)]
    id: u64,
  },
  Update {
    #[arg(short, long)]
    id: u64,
    #[arg(short, long, value_parser = NonEmptyStringValueParser::new())]
    title: Option<String>,
    #[arg(short, long)]
    content: Option<String>,
  },
  Delete {
    #[arg(short, long)]
    id: u64,
  },
  List {
    #[arg(short, long, default_value_t = 10)]
    limit: u64,
    #[arg(short, long, default_value_t = 0)]
    offset: u64,
  },
}

fn socket_address_validation(path: &str) -> Result<SocketAddr, CliError> {
  let addr: SocketAddr = path.parse().map_err(CliError::from)?;

  Ok(addr)
}
