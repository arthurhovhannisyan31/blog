use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::{
  error::ErrorUnauthorized, Error, FromRequest, HttpMessage, HttpRequest,
};

use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::jwt::JwtService;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
  pub user_id: i64,
  pub username: String,
}

impl FromRequest for AuthenticatedUser {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;

  fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    match req.extensions().get::<AuthenticatedUser>() {
      Some(user) => ready(Ok(user.clone())),
      None => ready(Err(ErrorUnauthorized("missing authenticated user"))),
    }
  }
}

pub async fn extract_user_from_token(
  token: &str,
  keys: &JwtService,
  auth_service: &AuthService<PostgresUserRepository>,
) -> Result<AuthenticatedUser, Error> {
  let claims = keys
    .verify_token(token)
    .map_err(|_| ErrorUnauthorized("invalid token"))?;

  let user = auth_service
    .get(claims.user_id)
    .await
    .map_err(|_| ErrorUnauthorized("user not found"))?;

  Ok(AuthenticatedUser {
    user_id: user.id,
    username: user.email,
  })
}
