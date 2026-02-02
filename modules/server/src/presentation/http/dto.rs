use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
  pub access_token: String,
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
  pub limit: Option<i64>,
  pub offset: Option<i64>,
}
