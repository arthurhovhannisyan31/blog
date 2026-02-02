use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
use tracing::info;

use crate::application::{auth_service::AuthService, error::ApplicationError};
use crate::data::user_repository::PostgresUserRepository;
use crate::presentation::http::dto::{
  CreateUserRequest, LoginRequest, TokenResponse,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(health).service(register).service(login);
}

#[post("/health")]
async fn health() -> impl Responder {
  HttpResponse::Ok().json(json!({
    "status": "ok",
    "timestamp": Utc::now(),
  }))
}

#[post("/auth/register")]
async fn register(
  service: web::Data<AuthService<PostgresUserRepository>>,
  payload: web::Json<CreateUserRequest>,
) -> Result<impl Responder, ApplicationError> {
  let user = service
    .register(
      payload.email.clone(),
      payload.username.clone(),
      payload.password.clone(),
    )
    .await?;

  info!(user_id = %user.id, email = %user.email, username = %user.username, "user registered");

  Ok(HttpResponse::Created().json(json!({
    "user_id": user.id,
    "email": user.email,
    "username": user.username
  })))
}

#[post("/auth/login")]
async fn login(
  service: web::Data<AuthService<PostgresUserRepository>>,
  payload: web::Json<LoginRequest>,
) -> Result<impl Responder, ApplicationError> {
  let jwt = service.login(&payload.email, &payload.password).await?;

  Ok(HttpResponse::Ok().json(TokenResponse { access_token: jwt }))
}
