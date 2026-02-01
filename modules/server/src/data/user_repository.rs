// use crate::domain::error::DomainError;
// use crate::domain::user::User;
// use async_trait::async_trait;
// use sqlx::PgPool;
// use tracing::info;
// use uuid::Uuid;
//
// #[async_trait]
// pub trait UserRepository: Send + Sync {
//   async fn create(&self, user: User) -> Result<User, DomainError>;
//   async fn find_by_email(
//     &self,
//     email: &str,
//   ) -> Result<Option<User>, DomainError>;
//   async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
// }
//
// #[derive(Clone)]
// pub struct PostgresUserRepository {
//   pool: PgPool,
// }
//
// #[async_trait]
// impl UserRepository for PostgresUserRepository {
//   async fn create(&self, user: User) -> Result<User, DomainError> {
//     sqlx::query(
//       r#"
//       INSERT INTO users (id, username, email, password_hash)
//       VALUES ($1, $2, $3, $4)
//     "#,
//     )
//     .bind(user.id)
//     .bind(user.username)
//     .bind(user.email)
//     .bind(user.password_hash)
//     .await;
//
//     info!(user_id = %user.id, email = %user.email, "user created");
//     Ok(user)
//   }
// }
