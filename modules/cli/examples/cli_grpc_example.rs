use cli::error::CliError;
use cli::init_client::init_client;
use cli::logging::init_logging;
use cli::token::{BLOG_TOKEN_PATH, read_token};

#[tokio::main]
async fn main() -> Result<(), CliError> {
  init_logging();

  let mut client = init_client(true, None).await?;
  let token = read_token(BLOG_TOKEN_PATH.to_string()).await?;
  client.set_token(token);

  // add cases login -> list

  // cargo run -- -g register --username "ivan" --email "ivan@example.com" --password "secret123"
  // cargo run -- -g login --email "ivan@example.com" --password "secret123"
  // cargo run -- -g create --title "Мой первый пост" --content "Содержание"
  // cargo run -- -g get --id 25
  // cargo run -- -g update --id 25 --title "Обновлённый заголовок"
  // cargo run -- -g delete --id 25
  // cargo run -- -g list --limit 20 --offset 0

  Ok(())
}
