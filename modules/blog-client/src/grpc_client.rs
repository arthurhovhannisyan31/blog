use std::error::Error;

use common::constants::{QUERY_LIMIT, QUERY_OFFSET};
use proto_generator::blog::{
  AuthRequest, AuthResponse, CreatePostRequest, CreateUserRequest,
  DeletePostRequest, GetPostRequest, ListPostsRequest, PostResponse,
  PostsListResponse, UpdatePostRequest,
  blog_protected_service_client::BlogProtectedServiceClient,
  blog_public_service_client::BlogPublicServiceClient,
};
use tonic::{Request, metadata::MetadataValue};

use crate::AbstractBlogClient;
use crate::error::BlogClientError;

pub struct GrpcBlogClient {
  pub public: BlogPublicServiceClient<tonic::transport::Channel>,
  pub protected: BlogProtectedServiceClient<tonic::transport::Channel>,
}

impl GrpcBlogClient {
  pub async fn new(addr: String) -> Result<Self, Box<dyn Error>> {
    let public = BlogPublicServiceClient::connect(addr.clone()).await?;
    let protected = BlogProtectedServiceClient::connect(addr).await?;

    Ok(Self { public, protected })
  }
}

impl AbstractBlogClient for GrpcBlogClient {
  async fn register(
    &mut self,
    username: String,
    email: String,
    password: String,
  ) -> Result<AuthResponse, BlogClientError> {
    let response = self
      .public
      .register(Request::new(CreateUserRequest {
        email,
        password,
        username,
      }))
      .await?;
    let data = response.into_inner();

    Ok(data)
  }
  async fn login(
    &mut self,
    email: String,
    password: String,
  ) -> Result<AuthResponse, BlogClientError> {
    let response = self
      .public
      .login(Request::new(AuthRequest { email, password }))
      .await?;
    let data = response.into_inner();

    Ok(data)
  }
  async fn create_post(
    &mut self,
    token: &str,
    title: String,
    content: String,
  ) -> Result<PostResponse, BlogClientError> {
    let metadata = MetadataValue::try_from(token)
      .map_err(|e| BlogClientError::Internal(e.to_string()))?;
    let mut request = Request::new(CreatePostRequest { content, title });
    request.metadata_mut().insert("authorization", metadata);
    let response = self.protected.create_post(request).await?;

    Ok(response.into_inner())
  }
  async fn get_post(
    &mut self,
    id: i64,
  ) -> Result<PostResponse, BlogClientError> {
    let response = self.public.get_post(GetPostRequest { id }).await?;

    Ok(response.into_inner())
  }
  async fn list_posts(
    &mut self,
    limit: Option<u64>,
    offset: Option<u64>,
  ) -> Result<PostsListResponse, BlogClientError> {
    let request = Request::new(ListPostsRequest {
      limit: limit.unwrap_or(QUERY_LIMIT) as i64,
      offset: offset.unwrap_or(QUERY_OFFSET) as i64,
    });
    let response = self.public.list_posts(request).await?;

    Ok(response.into_inner())
  }
  async fn update_post(
    &mut self,
    token: &str,
    id: i64,
    title: Option<String>,
    content: Option<String>,
  ) -> Result<PostResponse, BlogClientError> {
    let metadata = MetadataValue::try_from(token)
      .map_err(|e| BlogClientError::Internal(e.to_string()))?;
    let mut request = Request::new(UpdatePostRequest { id, content, title });
    request.metadata_mut().insert("authorization", metadata);
    let response = self.protected.update_post(request).await?;

    Ok(response.into_inner())
  }
  async fn delete_post(
    &mut self,
    token: &str,
    id: i64,
  ) -> Result<(), BlogClientError> {
    let metadata = MetadataValue::try_from(token).map_err(|e| {
      BlogClientError::Internal(format!("Failed building MetadataValue: {e:?}"))
    })?;
    let mut request = Request::new(DeletePostRequest { id });
    request.metadata_mut().insert("authorization", metadata);
    let _ = self.protected.delete_post(request).await?;

    Ok(())
  }
}
