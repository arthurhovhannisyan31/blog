use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/* Entities */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: i64,
  pub username: String,
  pub email: String,
  pub password_hash: String,
  pub created_at: DateTime<Utc>,
}

impl User {
  pub fn new(username: String, email: String, password_hash: String) -> Self {
    Self {
      id: 0,
      username,
      email,
      password_hash,
      created_at: Utc::now(),
    }
  }
}

/* DTO */
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
  pub username: String,
  pub password: String,
}
