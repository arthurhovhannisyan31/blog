use proto_generator::blog::{
  AuthRequest, AuthResponse, AuthenticatedUser, CreatePostRequest,
  CreateUserRequest, DeletePostRequest, EmptyResponse, GetPostRequest,
  PostResponse, StreamPostsRequest, UpdatePostRequest,
  blog_protected_service_server::BlogProtectedService,
  blog_public_service_server::BlogPublicService,
};
use std::time::Duration;
use tonic::codegen::tokio_stream;
use tonic::metadata::MetadataMap;
use tonic::{Code, Request, Response, Status};
use tracing::info;

use crate::application::{
  auth_service::AuthService, blog_service::BlogService,
};
use crate::data::{
  post_repository::PostgresPostRepository,
  user_repository::PostgresUserRepository,
};
use crate::presentation::grpc::constants::USER_ID_HEADER;
use crate::presentation::http::{dto, posts::ensure_owner};

#[derive(Clone)]
pub struct GrpcBlogPublicServiceImpl {
  auth_service: AuthService<PostgresUserRepository>,
  blog_service: BlogService<PostgresPostRepository>,
}

impl GrpcBlogPublicServiceImpl {
  pub fn new(
    auth_service: AuthService<PostgresUserRepository>,
    blog_service: BlogService<PostgresPostRepository>,
  ) -> Self {
    Self {
      auth_service,
      blog_service,
    }
  }
}

#[derive(Clone)]
pub struct GrpcBlogProtectedServiceImpl {
  auth_service: AuthService<PostgresUserRepository>,
  blog_service: BlogService<PostgresPostRepository>,
}

impl GrpcBlogProtectedServiceImpl {
  pub fn new(
    auth_service: AuthService<PostgresUserRepository>,
    blog_service: BlogService<PostgresPostRepository>,
  ) -> Self {
    Self {
      auth_service,
      blog_service,
    }
  }
}

#[tonic::async_trait]
impl BlogPublicService for GrpcBlogPublicServiceImpl {
  async fn register(
    &self,
    request: Request<CreateUserRequest>,
  ) -> Result<Response<AuthResponse>, Status> {
    let request = request.into_inner();
    let user = self
      .auth_service
      .register(
        request.email.clone(),
        request.password.clone(),
        request.username.clone(),
      )
      .await?;
    info!(user_id = %user.id, email = %user.email, username = %user.username, "user registered");

    let token = self
      .auth_service
      .login(&request.email, &request.password)
      .await?;

    Ok(Response::new(AuthResponse {
      user: Some(AuthenticatedUser::from(user)),
      token,
    }))
  }
  async fn login(
    &self,
    request: Request<AuthRequest>,
  ) -> Result<Response<AuthResponse>, Status> {
    let request = request.into_inner();
    let token = self
      .auth_service
      .login(&request.email, &request.password)
      .await?;
    let user = self.auth_service.get_by_email(&request.email).await?;
    let authenticated_user = AuthenticatedUser {
      user_id: user.id,
      email: user.email,
      username: user.username,
    };

    Ok(Response::new(AuthResponse {
      user: Some(authenticated_user),
      token,
    }))
  }
  async fn get_post(
    &self,
    request: Request<GetPostRequest>,
  ) -> Result<Response<PostResponse>, Status> {
    let request = request.into_inner();
    let post = self.blog_service.get_post(request.id).await?;

    Ok(Response::new(PostResponse::from(post)))
  }

  type StreamPostsStream =
    tokio_stream::wrappers::ReceiverStream<Result<PostResponse, Status>>;

  async fn stream_posts(
    &self,
    request: Request<StreamPostsRequest>,
  ) -> Result<Response<Self::StreamPostsStream>, Status> {
    let request = request.into_inner();
    let list = self
      .blog_service
      .list_posts(request.limit, request.offset)
      .await?;
    let mapped_list: Vec<PostResponse> =
      list.into_iter().map(PostResponse::from).collect();
    let (tx, rx) = tokio::sync::mpsc::channel(128);

    tokio::spawn(async move {
      for post in &mapped_list {
        tokio::time::sleep(Duration::from_secs(1)).await;

        if tx.send(Ok(post.clone())).await.is_err() {
          // Client disconnected
          return;
        }
      }
    });

    Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
      rx,
    )))
  }
}

fn read_user_id_from_metadata(metadata: &MetadataMap) -> Result<i64, Status> {
  let Some(user_id_value) = metadata.get(USER_ID_HEADER) else {
    return Err(Status::new(Code::Internal, "Failed reading user_id header"));
  };

  user_id_value
    .to_str()
    .map_err(|e| {
      Status::new(
        Code::Internal,
        format!("Failed reading user_id value: {}", e),
      )
    })?
    .parse::<i64>()
    .map_err(|e| {
      Status::new(
        Code::Internal,
        format!("Failed parsing user_id value: {}", e),
      )
    })
}

#[tonic::async_trait]
impl BlogProtectedService for GrpcBlogProtectedServiceImpl {
  async fn create_post(
    &self,
    request: Request<CreatePostRequest>,
  ) -> Result<Response<PostResponse>, Status> {
    let user_id = read_user_id_from_metadata(request.metadata())?;
    let user = self.auth_service.get(user_id).await?;
    let payload = request.into_inner();
    let post = self
      .blog_service
      .create_post(payload.title.clone(), payload.content.clone(), user.id)
      .await?;

    info!(
      user_id = %post.author_id,
      post_id = %post.id,
      title = %payload.title,
      content = %payload.content,
      "Post created: "
    );

    Ok(Response::new(PostResponse::from(post)))
  }
  async fn update_post(
    &self,
    request: Request<UpdatePostRequest>,
  ) -> Result<Response<PostResponse>, Status> {
    let user_id = read_user_id_from_metadata(request.metadata())?;
    let user = self.auth_service.get(user_id).await?;
    let payload = request.into_inner();
    let post = self
      .blog_service
      .update_post(payload.id, payload.title, payload.content, user.id)
      .await?;

    info!(
      user_id = %user.id,
      post_id = post.id,
      "Post updated"
    );

    Ok(Response::new(PostResponse::from(post)))
  }
  async fn delete_post(
    &self,
    request: Request<DeletePostRequest>,
  ) -> Result<Response<EmptyResponse>, Status> {
    let user_id = read_user_id_from_metadata(request.metadata())?;
    let user = self.auth_service.get(user_id).await?;
    let payload = request.into_inner();
    let post = self.blog_service.get_post(payload.id).await?;

    ensure_owner(&post, &dto::AuthenticatedUser::from(user))?;

    self.blog_service.delete_post(payload.id).await?;

    Ok(Response::new(EmptyResponse {}))
  }
}
