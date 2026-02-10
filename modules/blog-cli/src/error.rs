use std::io;
use std::net::AddrParseError;

use blog_client::error::BlogClientError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
  #[error("Parse addr error")]
  AddrParseError(#[from] AddrParseError),
  #[error("Blog client error")]
  BlogClientError(#[from] BlogClientError),
  #[error("IO Error")]
  IO(#[from] io::Error),
  #[error("Validation error: {0}")]
  ValidationError(String),
  #[error("Failed to read env variable")]
  VarError(String),
  #[error(transparent)]
  OtherError(#[from] anyhow::Error),
}
