use thiserror::Error;

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
}
