use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
  #[error("User not found: {0}")]
  UserNotFound(i64),
  #[error("User already exists: {0}")]
  UserAlreadyExists(i64),
  #[error("Invalid credentials")]
  InvalidCredentials,
  #[error("Post not found: {0}")]
  PostNotFound(u64),
  #[error("Access is forbidden")]
  Forbidden,
  #[error("internal error: {0}")]
  Internal(String),
}
