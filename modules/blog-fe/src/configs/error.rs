use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Request error")]
  RequestError(#[from] reqwest::Error),
  #[error(transparent)]
  OtherError(#[from] anyhow::Error),
}
