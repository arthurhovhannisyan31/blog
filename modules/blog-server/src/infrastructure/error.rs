use std::{io, net::AddrParseError, num::ParseIntError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
  #[error("Parse addr error")]
  AddrParseError(#[from] AddrParseError),
  #[error("IO Error")]
  IO(#[from] io::Error),
  #[error("Parse int error")]
  ParseIntError(#[from] ParseIntError),
  #[error("Sqlx error")]
  SqlxError(String),
  #[error("Failed to read env variable")]
  VarError(String),
  #[error(transparent)]
  OtherError(#[from] anyhow::Error),
}
