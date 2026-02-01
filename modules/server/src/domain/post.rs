use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/* Entities */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
  pub id: Uuid,
  pub title: String,
  pub content: String,
  pub author_id: Uuid,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Post {
  pub fn new(
    title: String,
    content: String,
    author_id: Uuid, // TODO Validate in handler
  ) -> Self {
    Self {
      id: Uuid::new_v4(),
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
