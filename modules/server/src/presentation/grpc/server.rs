use proto_generator::blog::{
  AuthRequest, AuthResponse, AuthenticatedUser, CreatePostRequest,
  CreateUserRequest, DeletePostRequest, DeletePostResponse, GetPostRequest,
  PostResponse, UpdatePostRequest,
  grpc_blog_protected_service_server::GrpcBlogProtectedService,
  grpc_blog_public_service_server::GrpcBlogPublicService,
};
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
impl GrpcBlogPublicService for GrpcBlogPublicServiceImpl {
  async fn register(
    &self,
    request: Request<CreateUserRequest>,
  ) -> Result<Response<AuthResponse>, Status> {
    let payload = request.into_inner();
    let user = self
      .auth_service
      .register(
        payload.email.clone(),
        payload.password.clone(),
        payload.username.clone(),
      )
      .await?;
    info!(user_id = %user.id, email = %user.email, username = %user.username, "user registered");

    let token = self
      .auth_service
      .login(&payload.email, &payload.password)
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
    let payload = request.into_inner();
    let token = self
      .auth_service
      .login(&payload.email, &payload.password)
      .await?;
    let user = self.auth_service.get_by_email(&payload.email).await?;
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
    let payload = request.into_inner();
    let post = self.blog_service.get_post(payload.id).await?;

    Ok(Response::new(PostResponse {
      id: post.id,
      title: post.title,
      content: post.content,
      author_id: post.author_id,
      created_at: post.created_at.timestamp(),
      updated_at: post.updated_at.timestamp(),
    }))
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
impl GrpcBlogProtectedService for GrpcBlogProtectedServiceImpl {
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
  ) -> Result<Response<DeletePostResponse>, Status> {
    let user_id = read_user_id_from_metadata(request.metadata())?;
    let user = self.auth_service.get(user_id).await?;
    let payload = request.into_inner();
    let post = self.blog_service.get_post(payload.id).await?;

    ensure_owner(&post, &dto::AuthenticatedUser::from(user))?;

    self.blog_service.delete_post(payload.id).await?;

    Ok(Response::new(DeletePostResponse {}))
  }
}
