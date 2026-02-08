pub mod client;
pub mod error;
pub mod grpc_client;
pub mod http_client;

use proto_generator::blog::{AuthResponse, ListPostResponse, PostResponse};

use crate::error::BlogClientError;

#[derive(Clone)]
pub enum Transport {
  Http(String),
  Grpc(String),
}

pub trait AbstractBlogClient: Sized {
  fn register(
    &mut self,
    username: String,
    email: String,
    password: String,
  ) -> impl Future<Output = Result<AuthResponse, BlogClientError>>;
  fn login(
    &mut self,
    username: String,
    password: String,
  ) -> impl Future<Output = Result<AuthResponse, BlogClientError>>;
  fn create_post(
    &mut self,
    token: &str,
    title: String,
    content: String,
  ) -> impl Future<Output = Result<PostResponse, BlogClientError>>;
  fn get_post(
    &mut self,
    id: i64,
  ) -> impl Future<Output = Result<PostResponse, BlogClientError>>;
  fn list_posts(
    &mut self,
    limit: Option<u64>,
    offset: Option<u64>,
  ) -> impl Future<Output = Result<ListPostResponse, BlogClientError>>;
  fn update_post(
    &mut self,
    token: &str,
    id: i64,
    title: String,
    content: String,
  ) -> impl Future<Output = Result<PostResponse, BlogClientError>>;
  fn delete_post(
    &mut self,
    token: &str,
    id: i64,
  ) -> impl Future<Output = Result<(), BlogClientError>>;
}
