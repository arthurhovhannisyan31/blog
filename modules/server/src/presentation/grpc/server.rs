use proto_generator::blog::{
  grpc_blog_service_server::GrpcBlogService, AuthRequest, AuthResponse, AuthenticatedUser,
  CreatePostRequest, CreateUserRequest, DeletePostRequest, DeletePostResponse,
  GetPostRequest, PostResponse, UpdatePostRequest,
};
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};
use tracing::info;

use crate::application::{
  auth_service::AuthService, blog_service::BlogService,
};
use crate::data::{
  post_repository::PostgresPostRepository,
  user_repository::PostgresUserRepository,
};
use crate::infrastructure::auth::extract_user_from_token;
use crate::infrastructure::jwt::JwtService;

#[derive(Clone)]
pub struct GrpcBlogServiceImpl {
  auth_service: AuthService<PostgresUserRepository>,
  blog_service: BlogService<PostgresPostRepository>,
  jwt_service: Arc<JwtService>,
}

impl GrpcBlogServiceImpl {
  pub fn new(
    auth_service: AuthService<PostgresUserRepository>,
    blog_service: BlogService<PostgresPostRepository>,
    jwt_service: Arc<JwtService>,
  ) -> Self {
    Self {
      auth_service,
      blog_service,
      jwt_service,
    }
  }
}

#[tonic::async_trait]
impl GrpcBlogService for GrpcBlogServiceImpl {
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
  async fn login(
    &self,
    request: Request<AuthRequest>,
  ) -> Result<Response<AuthResponse>, Status> {
    let metadata = request.metadata();
    let user_id = metadata.get("user_id");
    info!(user_id = ?user_id, "user_id from header omg!");

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
  async fn create_post(
    &self,
    request: Request<CreatePostRequest>,
  ) -> Result<Response<PostResponse>, Status> {
    let metadata = request.metadata();
    let header = metadata
      .get(actix_web::http::header::AUTHORIZATION.as_str())
      .ok_or_else(|| {
        Status::new(Code::Unauthenticated, "Missing authorization header")
      })?;
    let token = header
      .to_str()
      .map_err(|e| Status::new(Code::InvalidArgument, e.to_string()))?
      .strip_prefix("Bearer ")
      .ok_or_else(|| {
        Status::new(Code::InvalidArgument, "Failed reading token value")
      })?;
    let user =
      extract_user_from_token(token, &self.jwt_service, &self.auth_service)
        .await
        .map_err(|e| Status::new(Code::Unauthenticated, e.to_string()))?;

    let payload = request.into_inner();
    let post = self
      .blog_service
      .create_post(payload.title.clone(), payload.content.clone(), user.user_id)
      .await?;

    info!(
      user_id = %user.user_id,
      title = %payload.title,
      content = %payload.content,
      "Post created: "
    );

    Ok(Response::new(PostResponse {
      id: post.id,
      author_id: post.author_id,
      content: post.content,
      title: post.title,
      created_at: post.created_at.timestamp(),
      updated_at: post.updated_at.timestamp(),
    }))

    // add middleware to grpc server to return unauthorized
    // insert stripped token to request on middleware part
    //     // Добавление метаданных в ответ
    //     let mut response = Response::new(OrderResponse { order: Some(...) });
    //     response.metadata_mut().insert(
    //         "x-request-id",
    //         Uuid::new_v4().to_string().parse().unwrap(),
    //     );
    //
    //     Ok(response)
  }
  async fn get_post(
    &self,
    request: Request<GetPostRequest>,
  ) -> Result<Response<PostResponse>, Status> {
    todo!()
  }
  async fn update_post(
    &self,
    request: Request<UpdatePostRequest>,
  ) -> Result<Response<PostResponse>, Status> {
    todo!()
  }
  async fn delete_post(
    &self,
    request: Request<DeletePostRequest>,
  ) -> Result<Response<DeletePostResponse>, Status> {
    todo!()
  }
}
