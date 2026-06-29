use std::env;

fn main() {
  let docker_container =
    env::var("DOCKER_CONTAINER").unwrap_or("false".to_owned());
  // Load variables when run locally
  if &docker_container == "false" {
    dotenvy::dotenv().ok();
  }

  println!("cargo:rerun-if-changed=.env");
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/config.toml");

  let port = env::var("BACKEND_PORT").unwrap_or_else(|_| "8080".into());
  let host = env::var("BACKEND_HOST").unwrap_or_else(|_| "localhost".into());

  // Expose env variables to Dioxus app context
  println!("cargo::rustc-env=BACKEND_PORT={}", port);
  println!("cargo::rustc-env=BACKEND_HOST={}", host);
}
