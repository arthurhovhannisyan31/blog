use crate::error::CliError;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BLOG_TOKEN_PATH: &str = ".blog_token";

pub struct Token {
  token: String,
}

impl Token {
  pub async fn init() -> Result<Token, CliError> {
    let mut f = File::open(BLOG_TOKEN_PATH).await?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).await?;

    if buffer.is_empty() {
      return Err(CliError::ValidationError(
        "Token should not be empty".into(),
      ));
    }

    Ok(Self { token: buffer })
  }
  pub fn get(&self) -> &str {
    &self.token
  }
  pub async fn set(&mut self, token: String) -> Result<(), CliError> {
    self.token = token.clone();

    let mut f = File::open(BLOG_TOKEN_PATH).await?;
    let _ = f.write(token.as_bytes()).await?;

    Ok(())
  }
}
