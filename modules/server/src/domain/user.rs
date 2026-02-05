use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::presentation::http::dto::AuthenticatedUser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: i64,
  pub username: String,
  pub email: String,
  pub password_hash: String,
  pub created_at: DateTime<Utc>,
}

impl User {
  pub fn new(email: String, password_hash: String, username: String) -> Self {
    Self {
      id: 0,
      username,
      email,
      password_hash,
      created_at: Utc::now(),
    }
  }
}

impl From<User> for proto_generator::blog::AuthenticatedUser {
  fn from(user: User) -> Self {
    Self {
      email: user.email,
      user_id: user.id,
      username: user.username,
    }
  }
}

impl From<User> for AuthenticatedUser {
  fn from(user: User) -> Self {
    Self {
      email: user.email,
      user_id: user.id,
      username: user.username,
    }
  }
}
