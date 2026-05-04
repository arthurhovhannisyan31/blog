fn main() {
  dotenvy::dotenv().ok();

  println!("cargo:rerun-if-changed=.env");
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/config.toml");

  // TODO try
  // let path: &'static str = env!("PATH");
  let port = std::env::var("API_PORT").unwrap_or_else(|_| "8080".into());
  let host = std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".into());

  // Expose env variables to Dioxus app context
  println!("cargo::rustc-env=API_PORT={}", port);
  println!("cargo::rustc-env=API_HOST={}", host);
}
