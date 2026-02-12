use serde::Deserialize;

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
