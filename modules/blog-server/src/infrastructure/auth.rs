use actix_web::{Error, error::ErrorUnauthorized};

use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::jwt::JwtService;
use crate::presentation::http::dto::AuthenticatedUser;

pub async fn extract_user_from_token(
  token: &str,
  jwt_service: &JwtService,
  auth_service: &AuthService<PostgresUserRepository>,
) -> Result<AuthenticatedUser, Error> {
  let claims = jwt_service
    .verify_token(token)
    .map_err(|_| ErrorUnauthorized("invalid token"))?;

  let user = auth_service
    .get(claims.user_id)
    .await
    .map_err(|_| ErrorUnauthorized("user not found"))?;

  Ok(AuthenticatedUser {
    user_id: user.id,
    username: user.username,
    email: user.email,
  })
}
