use reqwest::Error;
use thiserror::Error;
use tonic;

#[derive(Debug, Error)]
pub enum BlogClientError {
  #[error("Http error")]
  Http(reqwest::Error),
  #[error("Grpc error")]
  Grpc(tonic::Status),
  #[error("Transport error")]
  Transport(tonic::transport::Error),
  #[error("Resource was not found")]
  NotFound,
  #[error("Request is unauthorized")]
  Unauthorized,
  #[error("Invalid request: {0}")]
  InvalidRequest(String),
  #[error("Internal: {0}")]
  Internal(String),
}

impl From<reqwest::Error> for BlogClientError {
  fn from(e: Error) -> Self {
    BlogClientError::Http(e)
  }
}

impl From<tonic::Status> for BlogClientError {
  fn from(status: tonic::Status) -> Self {
    BlogClientError::Grpc(status)
  }
}
