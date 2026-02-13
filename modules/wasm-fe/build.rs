use std::fs;
use std::path::Path;

fn main() {
  println!("cargo:rerun-if-changed=.env");
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src/config.toml");

  generate_env_constants();
}

fn generate_env_constants() {
  dotenvy::dotenv().ok();

  let out_dir = "./src";
  let dest = Path::new(&out_dir).join("generated.rs");

  let port = std::env::var("API_PORT").unwrap_or_else(|_| "8080".into());
  let host = std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".into());

  let code = format!(
    r#"pub const PORT: &str = {:?};
pub const HOST: &str = {:?};
"#,
    port, host
  );

  fs::write(dest, code.as_bytes()).unwrap();
}
