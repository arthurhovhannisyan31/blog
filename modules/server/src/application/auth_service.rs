use std::sync::Arc;

use crate::application::error::ApplicationError;
use crate::data::user_repository::UserRepository;
use crate::domain::user::User;
use crate::infrastructure::jwt::{hash_password, verify_password, JwtKeys};

#[derive(Clone)]
pub struct AuthService<R: UserRepository + 'static> {
  repo: Arc<R>,
  keys: JwtKeys,
}

impl<R> AuthService<R>
where
  R: UserRepository + 'static,
{
  pub fn new(repo: Arc<R>, keys: JwtKeys) -> Self {
    Self { repo, keys }
  }

  pub fn keys(&self) -> &JwtKeys {
    &self.keys
  }

  // TODO Check if it works
  // #[instrument(skip(self))]
  pub async fn get(&self, id: i64) -> Result<User, ApplicationError> {
    self
      .repo
      .find_by_id(id)
      .await?
      .ok_or_else(|| ApplicationError::NotFound(format!("user {}", id)))
  }

  // #[instrument(skip(self))]
  pub async fn register(
    &self,
    email: String,
    password: String,
    username: String,
  ) -> Result<User, ApplicationError> {
    let hash = hash_password(&password)
      .map_err(|err| ApplicationError::Internal(err.to_string()))?;
    let user = User::new(email.to_lowercase(), hash, username);

    self.repo.create(user).await.map_err(ApplicationError::from)
  }

  // #[instrument(skip(self))]
  pub async fn login(
    &self,
    email: &str,
    password: &str,
  ) -> Result<String, ApplicationError> {
    let user = self
      .repo
      .find_by_email(&email.to_lowercase())
      .await
      .map_err(ApplicationError::from)?
      .ok_or_else(|| ApplicationError::Unauthorized)?;

    let password_valid = verify_password(password, &user.password_hash)
      .map_err(|_| ApplicationError::Unauthorized)?;
    if !password_valid {
      return Err(ApplicationError::Unauthorized);
    }

    self
      .keys
      .generate_token(user.id)
      .map_err(|err| ApplicationError::Internal(err.to_string()))
  }
}
