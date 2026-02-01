use crate::domain::error::DomainError;
use crate::domain::post::Post;
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::error;

#[async_trait]
pub trait PostRepository: Send + Sync {
  async fn create(&self, post: Post) -> Result<Post, DomainError>;
  async fn get(&self, id: &str) -> Result<Option<Post>, DomainError>;
  async fn get_all(&self, owner_id: i64) -> Result<Vec<Post>, DomainError>;
  async fn update(&self, post: Post) -> Result<Post, DomainError>;
  async fn delete(&self, id: i64) -> Result<(), DomainError>;
}

#[derive(Clone)]
pub struct PostgresPostRepository {
  pool: PgPool,
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
  async fn create(&self, post: Post) -> Result<Post, DomainError> {
    let post_data = sqlx::query_as!(
      Post,
      r#"
        INSERT INTO posts (title, content, author_id)
        VALUES ($1, $2, $3)
        RETURNING posts.id, posts.title, posts.content, posts.author_id, posts.created_at, posts.updated_at
      "#,
      post.title,
      post.content,
      post.author_id,
    )
      .fetch_one(&self.pool)
      .await
      .map_err(|e| {
        error!("Failed to create post: {}", e);

        DomainError::Internal(format!("database error: {}", e))
      })?;

    Ok(post_data)
  }
  async fn delete(&self, id: i64) -> Result<(), DomainError> {
    todo!()
  }
  async fn get(&self, id: &str) -> Result<Option<Post>, DomainError> {
    todo!()
  }
  async fn get_all(&self, owner_id: i64) -> Result<Vec<Post>, DomainError> {
    todo!()
  }
  async fn update(&self, post: Post) -> Result<Post, DomainError> {
    todo!()
  }
}
