use std::sync::Arc;

use tonic::body::Body;
use tonic::codegen::http::{HeaderValue, Request};
use tonic::{async_trait, Code, Status};
use tonic_middleware::RequestInterceptor;

use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::{auth::extract_user_from_token, jwt::JwtService};
use crate::presentation::grpc::constants::USER_ID_HEADER;

#[async_trait]
pub trait AuthValidationService: Send + Sync + 'static {
  async fn verify_token(&self, token: &str) -> Result<i64, Status>;
}

#[derive(Clone)]
pub struct AuthValidationServiceImpl {
  auth_service: AuthService<PostgresUserRepository>,
  jwt_service: Arc<JwtService>,
}

impl AuthValidationServiceImpl {
  pub fn new(
    auth_service: AuthService<PostgresUserRepository>,
    jwt_service: Arc<JwtService>,
  ) -> Self {
    Self {
      auth_service,
      jwt_service,
    }
  }
}

#[async_trait]
impl AuthValidationService for AuthValidationServiceImpl {
  async fn verify_token(&self, token: &str) -> Result<i64, Status> {
    let token_value = token.strip_prefix("Bearer ").ok_or_else(|| {
      Status::new(Code::InvalidArgument, "Failed reading token value")
    })?;
    let user = extract_user_from_token(
      token_value,
      &self.jwt_service,
      &self.auth_service,
    )
    .await
    .map_err(|e| Status::new(Code::Unauthenticated, e.to_string()))?;

    Ok(user.user_id)
  }
}

#[derive(Clone)]
pub struct AuthInterceptor<A: AuthValidationService> {
  pub auth_service: A,
}

#[async_trait]
impl<A: AuthValidationService> RequestInterceptor for AuthInterceptor<A> {
  async fn intercept(
    &self,
    mut req: Request<Body>,
  ) -> Result<Request<Body>, Status> {
    match req
      .headers()
      .get(actix_web::http::header::AUTHORIZATION.as_str())
      .map(|v| v.to_str())
    {
      Some(Ok(token)) => {
        // Get user id from the token
        let user_id = self.auth_service.verify_token(token).await?;

        // Set user id in header, so it can be used in grpc services through tonic::Request::metadata()
        let user_id_header_value = HeaderValue::from_str(&user_id.to_string())
          .map_err(|_e| {
            Status::internal("Failed to convert user_id to header value")
          })?;
        req
          .headers_mut()
          .insert(USER_ID_HEADER, user_id_header_value);
        Ok(req)
      }
      _ => Err(Status::unauthenticated("Unauthenticated")),
    }
  }
}
