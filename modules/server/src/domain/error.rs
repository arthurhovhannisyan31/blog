use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
  #[error("User not found: {0}")]
  UserNotFound(uuid::Uuid),
  #[error("User already exists: {0}")]
  UserAlreadyExists(uuid::Uuid),
  #[error("Invalid credentials")]
  InvalidCredentials,
  #[error("Post not found: {0}")]
  PostNotFound(uuid::Uuid),
  #[error("Access is forbidden")]
  Forbidden,
}
