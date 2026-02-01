use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/* Entities */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
  pub id: u64,
  pub title: String,
  pub content: String,
  pub author_id: u64,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Post {
  pub fn new(
    title: String,
    content: String,
    author_id: u64, // TODO Validate in handler
  ) -> Self {
    Self {
      id: 0,
      title,
      content,
      author_id,
      created_at: Utc::now(),
      updated_at: Utc::now(),
    }
  }
}

/* DTO */
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
  pub title: String,
  pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
  pub title: String,
  pub content: String,
}
