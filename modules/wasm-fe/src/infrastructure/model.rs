use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostResponse {
  pub id: i64,
  pub title: String,
  pub content: String,
  pub author_id: i64,
  pub created_at: i64,
  pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct PostsListResponse {
  pub posts: Vec<PostResponse>,
  pub total: u64,
  pub limit: u64,
  pub offset: u64,
}

#[derive(Debug, Serialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthenticatedUser {
  pub email: String,
  pub user_id: i64,
  pub username: String,
}
#[derive(Debug, Serialize)]
pub struct AuthRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthResponse {
  pub token: String,
  pub user: AuthenticatedUser,
}
#[derive(Debug, Serialize)]
pub struct UpdatePostRequest {
  pub title: Option<String>,
  pub content: Option<String>,
}
