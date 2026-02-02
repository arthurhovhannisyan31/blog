use actix_cors::Cors;

use crate::infrastructure::config::AppConfig;

pub fn build_cors(config: &AppConfig) -> Cors {
  let mut cors = Cors::default()
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
    .allowed_headers(vec![
      actix_web::http::header::CONTENT_TYPE,
      actix_web::http::header::AUTHORIZATION,
    ])
    .supports_credentials()
    .max_age(3600);

  for origin in &config.cors_origins {
    cors = cors.allowed_origin(origin);
  }

  cors
}
