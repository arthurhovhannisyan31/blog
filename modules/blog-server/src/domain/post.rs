use chrono::{DateTime, Utc};
use proto_generator::blog::PostResponse;
use serde::{Deserialize, Serialize};

use crate::domain::error::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
  pub id: i64,
  pub title: String,
  pub content: String,
  pub author_id: i64,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Post {
  pub fn new(
    title: String,
    content: String,
    author_id: i64,
  ) -> Result<Self, DomainError> {
    if title.is_empty() {
      return Err(DomainError::Validation("Post title is required!".into()));
    }

    Ok(Self {
      id: 0,
      title,
      content,
      author_id,
      created_at: Utc::now(),
      updated_at: Utc::now(),
    })
  }
}

impl From<Post> for PostResponse {
  fn from(post: Post) -> Self {
    PostResponse {
      id: post.id,
      author_id: post.author_id,
      content: post.content,
      title: post.title,
      created_at: post.created_at.timestamp(),
      updated_at: post.updated_at.timestamp(),
    }
  }
}
