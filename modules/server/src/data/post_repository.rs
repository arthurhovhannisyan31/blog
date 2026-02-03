use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use tracing::error;

use crate::domain::error::DomainError;
use crate::domain::post::Post;

#[async_trait]
pub trait PostRepository: Send + Sync {
  async fn create(&self, post: Post) -> Result<Post, DomainError>;
  async fn get(&self, id: i64) -> Result<Option<Post>, DomainError>;
  async fn get_all(
    &self,
    limit: i64,
    offset: i64,
  ) -> Result<Vec<Post>, DomainError>;
  async fn get_row_count(&self) -> Result<i64, DomainError>;
  async fn update(&self, id: i64, post: Post) -> Result<Post, DomainError>;
  async fn delete(&self, id: i64) -> Result<(), DomainError>;
}

#[derive(Clone)]
pub struct PostgresPostRepository {
  pool: PgPool,
}

impl PostgresPostRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
  async fn create(&self, post: Post) -> Result<Post, DomainError> {
    let row = sqlx::query_as!(
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

    Ok(row)
  }
  async fn delete(&self, id: i64) -> Result<(), DomainError> {
    sqlx::query(
      r#"
        DELETE
        FROM posts
        WHERE posts.id = $1
      "#,
    )
    .bind(id)
    .fetch_optional(&self.pool)
    .await
    .map_err(|e| {
      error!("Failed to delete post: {}", e);
      // TODO Ensure 404 returned for missing post
      error!(
        constraint = e.as_database_error().and_then(|db| db.constraint()),
        "DB Constraint: Delete: "
      );

      DomainError::Internal(format!("database error: {}", e))
    })?;

    Ok(())
  }
  async fn get(&self, id: i64) -> Result<Option<Post>, DomainError> {
    let row = sqlx::query_as!(
      Post,
      r#"
        SELECT posts.id, posts.title, posts.content, posts.author_id, posts.created_at, posts.updated_at
        from posts
        WHERE posts.id = $1
      "#,
      id
    ).fetch_optional(&self.pool)
      .await
      .map_err(|e| {
        error!("Failed to fetch post: {}", e);
        error!(constraint = e
        .as_database_error()
        .and_then(|db| db.constraint()), "DB Constraint: Get: ");

        DomainError::Internal(format!("database error: {}", e))
      })?;

    Ok(row)
  }
  async fn get_all(
    &self,
    limit: i64,
    offset: i64,
  ) -> Result<Vec<Post>, DomainError> {
    let rows = sqlx::query_as!(
      Post,
      r#"
        SELECT *
        from posts
        LIMIT $1 OFFSET $2;
      "#,
      limit,
      offset,
    )
    .fetch_all(&self.pool)
    .await
    .map_err(|e| {
      error!("Failed to fetch posts: {}", e);
      error!(
        constraint = e.as_database_error().and_then(|db| db.constraint()),
        "DB Constraint: Get *: "
      );

      DomainError::Internal(format!("database error: {}", e))
    })?;

    Ok(rows)
  }
  async fn get_row_count(&self) -> Result<i64, DomainError> {
    let count = sqlx::query_scalar!("SELECT COUNT(*) from posts",)
      .fetch_one(&self.pool)
      .await
      .map_err(|e| {
        error!("Failed to fetch posts: {}", e);
        error!(
          constraint = e.as_database_error().and_then(|db| db.constraint()),
          "DB Constraint: Get *: "
        );

        DomainError::Internal(format!("database error: {}", e))
      })?;

    Ok(count.unwrap_or(0))
  }
  async fn update(&self, id: i64, post: Post) -> Result<Post, DomainError> {
    let row = sqlx::query_as!(
      Post,
      r#"
        UPDATE posts
        SET title = $2,
            content = $3,
            created_at = NOW()
        WHERE id = $1
        RETURNING posts.id, posts.title, posts.content, posts.author_id, posts.created_at, posts.updated_at
      "#,
      id,
      post.title,
      post.content,
    ).fetch_one(&self.pool)
      .await
      .map_err(|e| {
        error!("Failed to update posts: {}", e);

        // TODO return 404 if post not found
        error!(
        constraint = e.as_database_error().and_then(|db| db.constraint()),
        "DB Constraint: Get *: "
      );

        DomainError::Internal(format!("database error: {}", e))
      })?;

    Ok(row)
  }
}
