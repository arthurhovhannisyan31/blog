use std::sync::Arc;

use actix_web::{
<<<<<<< HEAD
  dev::Server, middleware::{DefaultHeaders, Logger},
  web,
  App,
  HttpServer,
||||||| parent of 701cf21 (add tls feature for blog-server)
=======
  App, HttpServer,
  dev::Server,
  middleware::{DefaultHeaders, Logger},
  web,
>>>>>>> 701cf21 (add tls feature for blog-server)
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
  auth_service: Arc<AuthService<PostgresUserRepository>>,
  blog_service: Arc<BlogService<PostgresPostRepository>>,
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
  });

  #[cfg(feature = "tls")]
  {
    use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    builder.set_private_key_file(config.tls_key_path, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(config.tls_crt_path)?;

    let server = server
      .bind_openssl((config.host.as_str(), config.http_port), builder)?
      .run();

    Ok(server)
  }

  #[cfg(not(feature = "tls"))]
  {
    let server = server.bind((config.host.as_str(), config.http_port))?.run();

    Ok(server)
  }
}
