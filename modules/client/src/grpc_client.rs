use std::error::Error;

use common::constants::{POST_STREAM_LIMIT, POST_STREAM_OFFSET};
use proto_generator::blog::{
  AuthRequest, AuthResponse, CreatePostRequest, CreateUserRequest,
  DeletePostRequest, EmptyResponse, GetPostRequest, PostResponse,
  StreamPostsRequest, UpdatePostRequest,
  blog_protected_service_client::BlogProtectedServiceClient,
  blog_public_service_client::BlogPublicServiceClient,
};
use tonic::{Request, metadata::MetadataValue};

use crate::client::BlogClientImpl;

pub struct GrpcBlogClient {
  pub public: BlogPublicServiceClient<tonic::transport::Channel>,
  pub protected: BlogProtectedServiceClient<tonic::transport::Channel>,
  pub token: Option<String>,
}

impl GrpcBlogClient {
  fn set_token(&mut self, token: String) {
    self.token = Some(format!("Bearer {}", token));
  }
  pub async fn new(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
    let public = BlogPublicServiceClient::connect(addr.clone()).await?;
    let protected = BlogProtectedServiceClient::connect(addr).await?;

    Ok(Self {
      public,
      protected,
      token: None,
    })
  }
}

impl BlogClientImpl<tonic::Streaming<PostResponse>, EmptyResponse>
  for GrpcBlogClient
{
  async fn register(
    &mut self,
    username: String,
    email: String,
    password: String,
  ) -> Result<AuthResponse, Box<dyn Error>> {
    let response = self
      .public
      .register(Request::new(CreateUserRequest {
        email,
        password,
        username,
      }))
      .await?;
    let data = response.into_inner();

    self.set_token(data.token.clone());

    Ok(data)
  }
  async fn login(
    &mut self,
    email: String,
    password: String,
  ) -> Result<AuthResponse, Box<dyn Error>> {
    let response = self
      .public
      .login(Request::new(AuthRequest { email, password }))
      .await?;
    let data = response.into_inner();

    self.set_token(data.token.clone());

    Ok(data)
  }
  async fn create_post(
    &mut self,
    title: String,
    content: String,
  ) -> Result<PostResponse, Box<dyn Error>> {
    let token_value = self.token.clone().unwrap_or_default();
    let mut request = Request::new(CreatePostRequest { content, title });
    request
      .metadata_mut()
      .insert("authorization", MetadataValue::try_from(token_value)?);
    let response = self.protected.create_post(request).await?;

    Ok(response.into_inner())
  }
  async fn get_post(
    &mut self,
    id: i64,
  ) -> Result<PostResponse, Box<dyn Error>> {
    let response = self.public.get_post(GetPostRequest { id }).await?;

    Ok(response.into_inner())
  }
  async fn list_posts(
    &mut self,
    limit: Option<i64>,
    offset: Option<i64>,
  ) -> Result<tonic::Streaming<PostResponse>, Box<dyn Error>> {
    let request = Request::new(StreamPostsRequest {
      limit: limit.unwrap_or(POST_STREAM_LIMIT),
      offset: offset.unwrap_or(POST_STREAM_OFFSET),
    });
    let response = self.public.stream_posts(request).await?;

    Ok(response.into_inner())
  }
  async fn update_post(
    &mut self,
    id: i64,
    title: String,
    content: String,
  ) -> Result<PostResponse, Box<dyn Error>> {
    let token_value = self.token.clone().unwrap_or_default();
    let mut request = Request::new(UpdatePostRequest { id, content, title });
    request
      .metadata_mut()
      .insert("authorization", MetadataValue::try_from(token_value)?);
    let response = self.protected.update_post(request).await?;

    Ok(response.into_inner())
  }
  async fn delete_post(
    &mut self,
    id: i64,
  ) -> Result<EmptyResponse, Box<dyn Error>> {
    let token_value = self.token.clone().unwrap_or_default();
    let mut request = Request::new(DeletePostRequest { id });
    request
      .metadata_mut()
      .insert("authorization", MetadataValue::try_from(token_value)?);
    let response = self.protected.delete_post(request).await?;

    Ok(response.into_inner())
  }
}
