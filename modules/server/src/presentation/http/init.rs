use std::sync::Arc;

use actix_web::{
  App, HttpServer,
  dev::Server,
  middleware::{DefaultHeaders, Logger},
  web,
};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::application::{
  auth_service::AuthService, blog_service::BlogService,
};
use crate::data::{
  post_repository::PostgresPostRepository,
  user_repository::PostgresUserRepository,
};
use crate::infrastructure::{
  config::AppConfig, cors::build_cors, jwt::JwtService,
};
use crate::presentation::{
  http::scoped::{protected_scope, public_scope},
  middleware::jwt_validator,
};

pub fn init_http_server(
  auth_service: AuthService<PostgresUserRepository>,
  blog_service: BlogService<PostgresPostRepository>,
  jwt_service: Arc<JwtService>,
  config: AppConfig,
) -> std::io::Result<Server> {
  let server = HttpServer::new(move || {
    let cors = build_cors(&config.cors_origins);
    let auth = HttpAuthentication::with_fn(jwt_validator);

    App::new()
      .wrap(Logger::default())
      .wrap(
        DefaultHeaders::new()
          .add(("X-Content-Type-Options", "nosniff"))
          .add(("Referrer-Policy", "no-referrer"))
          .add(("Permissions-Policy", "geolocation=()"))
          .add(("Cross-Origin-Opener-Policy", "same-origin")),
      )
      .wrap(cors)
      .app_data(web::Data::new(blog_service.clone()))
      .app_data(web::Data::new(auth_service.clone()))
      .app_data(web::Data::new(jwt_service.clone()))
      .service(
        web::scope("/api")
          .service(public_scope())
          .service(web::scope("").wrap(auth).service(protected_scope())),
      )
  })
  .bind((config.host.as_str(), config.http_port))?
  .run();

  Ok(server)
}
