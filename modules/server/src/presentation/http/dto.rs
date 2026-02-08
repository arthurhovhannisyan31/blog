use std::future::{ready, Ready};

use actix_web::{
  dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpMessage,
  HttpRequest,
};
use serde::{Deserialize, Serialize};

use crate::domain::post::Post;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
  pub id: i64,
  pub title: String,
  pub content: String,
  pub author_id: i64,
  pub created_at: i64,
  pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPostResponse {
  pub posts: Vec<PostResponse>,
  pub total: u64,
  pub limit: u64,
  pub offset: u64,
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

impl From<Post> for PostResponse {
  fn from(post: Post) -> Self {
    Self {
      id: post.id,
      title: post.title,
      content: post.content,
      author_id: post.author_id,
      created_at: post.created_at.timestamp(),
      updated_at: post.updated_at.timestamp(),
    }
  }
}
