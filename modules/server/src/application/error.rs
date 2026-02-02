use crate::domain::error::DomainError;
use actix_web::body::BoxBody;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
  #[error("Bad request: {0}")]
  BadRequest(String),
  #[error("Forbidden")]
  Forbidden,
  #[error("Internal server error: {0}")]
  Internal(String),
  #[error("Not found: {0}")]
  NotFound(String),
  #[error("Unauthorized")]
  Unauthorized,
  #[error("validation error: {0}")]
  Validation(String),
}

#[derive(Serialize)]
struct ErrorBody<'a> {
  error: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  details: Option<serde_json::Value>,
}

impl ResponseError for ApplicationError {
  fn status_code(&self) -> StatusCode {
    match self {
      ApplicationError::BadRequest(_) => StatusCode::BAD_REQUEST,
      ApplicationError::Forbidden => StatusCode::FORBIDDEN,
      ApplicationError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
      ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
      ApplicationError::Unauthorized => StatusCode::UNAUTHORIZED,
      ApplicationError::Validation(_) => StatusCode::BAD_REQUEST,
    }
  }

  fn error_response(&self) -> HttpResponse<BoxBody> {
    let message = self.to_string();
    let details = match self {
      ApplicationError::BadRequest(msg) => Some(json!({"message": msg})),
      ApplicationError::Forbidden => None,
      ApplicationError::Internal(msg) => Some(json!({"message": msg})),
      ApplicationError::NotFound(resource) => {
        Some(json!({"resource": resource}))
      }
      ApplicationError::Unauthorized => None,
      ApplicationError::Validation(msg) => Some(json!({"message": msg})),
    };
    let body = ErrorBody {
      error: &message,
      details,
    };
    HttpResponse::build(self.status_code()).json(body)
  }
}

impl From<DomainError> for ApplicationError {
  fn from(value: DomainError) -> Self {
    match value {
      DomainError::Forbidden => ApplicationError::Forbidden,
      DomainError::InvalidCredentials => ApplicationError::Unauthorized,
      DomainError::Internal(msg) => ApplicationError::Internal(msg),
      DomainError::PostNotFound(id) => {
        ApplicationError::NotFound(format!("Post not found: {}", id))
      }
      DomainError::UserAlreadyExists(id) => {
        ApplicationError::BadRequest(format!("User already exists: {}", id))
      }
      DomainError::UserNotFound(id) => {
        ApplicationError::NotFound(format!("User not found: {}", id))
      }
      DomainError::Validation(msg) => ApplicationError::Validation(msg),
    }
  }
}
