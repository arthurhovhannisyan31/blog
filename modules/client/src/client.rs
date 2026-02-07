use std::future::Future;

use proto_generator::blog::{AuthResponse, EmptyResponse, PostResponse};

pub trait BlogClientImpl: Sized {
  // fn new(transport: Transport) -> Self;
  fn new(
    addr: String,
  ) -> impl Future<Output = Result<Self, Box<dyn std::error::Error>>>;
  fn register(
    &mut self,
    username: String,
    email: String,
    password: String,
  ) -> impl Future<Output = Result<AuthResponse, Box<dyn std::error::Error>>>;
  fn login(
    &mut self,
    username: String,
    password: String,
  ) -> impl Future<Output = Result<AuthResponse, Box<dyn std::error::Error>>>;
  fn create_post(
    &mut self,
    title: String,
    content: String,
  ) -> impl Future<Output = Result<PostResponse, Box<dyn std::error::Error>>>;
  fn get_post(
    &mut self,
    id: i64,
  ) -> impl Future<Output = Result<PostResponse, Box<dyn std::error::Error>>>;
  fn update_post(
    &mut self,
    id: i64,
    title: String,
    content: String,
  ) -> impl Future<Output = Result<PostResponse, Box<dyn std::error::Error>>>;
  fn delete_post(
    &mut self,
    id: i64,
  ) -> impl Future<Output = Result<EmptyResponse, Box<dyn std::error::Error>>>;
  fn list_posts(
    &mut self,
    limit: Option<i64>,
    offset: Option<i64>,
  ) -> impl Future<
    Output = Result<tonic::Streaming<PostResponse>, Box<dyn std::error::Error>>,
  >;
}
