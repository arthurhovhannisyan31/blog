use std::sync::Arc;

use actix_web::{Error, HttpMessage, dev::ServiceRequest, error, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::{auth::extract_user_from_token, jwt::JwtService};

pub async fn jwt_validator(
  req: ServiceRequest,
  credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
  let Some(credentials) = credentials else {
    return Err((error::ErrorBadRequest("no bearer header"), req));
  };

  let Some(auth_service) = req
    .app_data::<web::Data<AuthService<PostgresUserRepository>>>()
    .cloned()
  else {
    return Err((
      error::ErrorInternalServerError("AuthService is missing"),
      req,
    ));
  };
  let Some(jwt_service) = req.app_data::<web::Data<Arc<JwtService>>>().cloned()
  else {
    return Err((
      error::ErrorInternalServerError("JwtService is missing"),
      req,
    ));
  };

  if credentials.token().is_empty() {
    return Err((error::ErrorUnauthorized("Missing jwt token"), req));
  }

  let user = extract_user_from_token(
    credentials.token(),
    jwt_service.get_ref(),
    auth_service.get_ref(),
  )
  .await;

  let authenticated_user = match user {
    Ok(user) => user,
    Err(err) => {
      return Err((err, req));
    }
  };

  req.extensions_mut().insert(authenticated_user);

  Ok(req)
}
