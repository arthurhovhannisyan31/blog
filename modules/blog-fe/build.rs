use std::env;

fn main() {
  dotenvy::dotenv().ok();

  println!("cargo:rerun-if-changed=.env");
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/config.toml");

  let port = env::var("BACKEND_PORT").unwrap_or_else(|_| "8080".into());
  let host = env::var("BACKEND_HOST").unwrap_or_else(|_| "127.0.0.1".into());
  let tls = env::var("BACKEND_TLS").unwrap_or("false".into());

  // Expose env variables to Dioxus app context
  println!("cargo::rustc-env=BACKEND_PORT={}", port);
  println!("cargo::rustc-env=BACKEND_HOST={}", host);
  println!("cargo::rustc-env=BACKEND_TLS={}", tls);
}
