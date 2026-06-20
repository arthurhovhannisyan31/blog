use std::sync::Arc;

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
use actix_web::{
  App, HttpServer,
  dev::Server,
  middleware::{DefaultHeaders, Logger},
  web,
};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn init_http_server(
  auth_service: Arc<AuthService<PostgresUserRepository>>,
  blog_service: Arc<BlogService<PostgresPostRepository>>,
  jwt_service: Arc<JwtService>,
  app_config: AppConfig,
) -> std::io::Result<Server> {
  let cors_origin = app_config.cors_origins.clone();

  let auth = Arc::new(HttpAuthentication::with_fn(jwt_validator));

  let server =
    HttpServer::new(move || {
      let cors = build_cors(&cors_origin);

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
        .service(web::scope("/api").service(public_scope()).service(
          web::scope("").wrap(auth.clone()).service(protected_scope()),
        ))
    });

  #[cfg(feature = "tls")]
  {
    use crate::presentation::http::tls::build_http_tls_config;

    let tls_config = build_http_tls_config(&app_config)
      .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let server = server
      .bind_rustls_0_23(
        (app_config.host.as_str(), app_config.http_port),
        tls_config,
      )?
      .run();

    Ok(server)
  }

  #[cfg(not(feature = "tls"))]
  {
    let server = server
      .bind((app_config.host.as_str(), app_config.http_port))?
      .run();

    Ok(server)
  }
}
