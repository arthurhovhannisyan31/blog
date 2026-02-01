use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{error, info};

use crate::domain::error::DomainError;
use crate::domain::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn create(&self, user: User) -> Result<User, DomainError>;
  async fn find_by_email(
    &self,
    email: &str,
  ) -> Result<Option<User>, DomainError>;
  async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
  pool: PgPool,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
  async fn create(&self, user: User) -> Result<User, DomainError> {
    let user_data = sqlx::query_as!(
      User,
      r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING users.id, users.username, users.email, users.password_hash, users.created_at
      "#,
      user.username,
      user.email,
      user.password_hash,
    )
    .fetch_one(&self.pool)
    .await
    .map_err(|e| {
      error!("Failed to create user: {}", e);

      if e
        .as_database_error()
        .and_then(|db| db.constraint())
        // TODO test: username unique, email unique
        .map(|c| c.contains("users_email")) // TODO Test string intersection
        == Some(true)
      {
        DomainError::UserAlreadyExists(user.id)
      } else {
        DomainError::Internal(format!("database error: {}", e))
      }
    })?;

    info!(user_id = %user.id, email = %user.email, "user created");
    Ok(user_data)
  }
  async fn find_by_email(
    &self,
    email: &str,
  ) -> Result<Option<User>, DomainError> {
    let user_data = sqlx::query_as!(
      User,
      r#"
        SELECT users.id, users.username, users.email, users.password_hash, users.created_at
        FROM users
        WHERE users.email = $1
      "#,
      email
    ).fetch_optional(&self.pool)
      .await
      .map_err(|e|{
        error!("Failed to find user by email {}: {}", email, e);
        DomainError::Internal(format!("database error: {}", e))
      })?;

    Ok(user_data)
  }
  async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
    let user_data = sqlx::query_as!(
      User,
      r#"
        SELECT users.id, users.username, users.email, users.password_hash, users.created_at
        FROM users
        WHERE users.id = $1
      "#,
      id
    ).fetch_optional(&self.pool)
      .await
      .map_err(|e|{
        error!("Failed to find user by id {}: {}", id, e);
        DomainError::Internal(format!("database error: {}", e))
      })?;

    Ok(user_data)
  }
}
