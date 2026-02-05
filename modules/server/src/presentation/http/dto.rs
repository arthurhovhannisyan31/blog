use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthenticatedUser {
  pub email: String,
  pub user_id: i64,
  pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
  pub token: String,
  pub user: AuthenticatedUser,
}

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

#[derive(Debug, Deserialize)]
pub struct GetPostsQueryParams {
  pub limit: Option<u64>,
  pub offset: Option<u64>,
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
