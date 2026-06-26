use actix_cors::Cors;

pub fn build_cors(cors_origins: &[String]) -> Cors {
  let mut cors = Cors::default()
    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    .allowed_headers(vec![
      actix_web::http::header::CONTENT_TYPE,
      actix_web::http::header::AUTHORIZATION,
    ])
    .supports_credentials()
    .max_age(3600);

  cors = cors.allow_any_origin();
  // for origin in cors_origins {
  //   cors = cors.allowed_origin(origin);
  //   cors = cors.allowed_origin(origin);
  // }

  cors
}
