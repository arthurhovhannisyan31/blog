use std::path::PathBuf;
use tokio::{
  fs::{File, OpenOptions},
  io::{AsyncReadExt, AsyncWriteExt},
};

use crate::error::CliError;

pub const BLOG_TOKEN_PATH: &str = ".blog_token";

pub async fn read_token(path: String) -> Result<String, CliError> {
  let path = PathBuf::from(path);
  let file_exists = path.exists();
  let mut buffer = String::new();

  if let Ok(mut f) = File::open(BLOG_TOKEN_PATH).await {
    f.read_to_string(&mut buffer).await?;
  }

  if file_exists && buffer.is_empty() {
    return Err(CliError::ValidationError(format!(
      "Token is missing in file: {BLOG_TOKEN_PATH}"
    )));
  }

  Ok(buffer)
}

pub async fn save_token(path: String, token: String) -> Result<(), CliError> {
  let mut f = OpenOptions::new()
    .create(true)
    .read(true)
    .write(true)
    .open(path)
    .await?;
  let _ = f.write(token.as_bytes()).await?;
  f.flush().await?;

  Ok(())
}
