use std::env;

pub fn get_env_protocol() -> String {
  let use_secure_connection =
    env::var("BACKEND_TLS").unwrap_or("false".into()).eq("true");

  (if use_secure_connection {
    "https"
  } else {
    "http"
  })
  .to_string()
}
