use blog_cli::error::CliError;
use blog_cli::init_client::init_client;
use blog_cli::logging::init_logging;
use blog_cli::token::{BLOG_TOKEN_PATH, read_token};
use tokio::process::Command;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), CliError> {
  init_logging();

  let mut client = init_client(false, None).await?;
  let token = read_token(BLOG_TOKEN_PATH.to_string()).await?;
  client.set_token(token);

  // try to login, if doesn't exist register

  let mut cmd = Command::new("../../target/release/blog-cli");
  let res = cmd
    .arg("register")
    .arg("--username")
    .arg("ivan")
    .arg("--email")
    .arg("ivan@example.com")
    .arg("--password")
    .arg("secret123")
    .output()
    .await;
  info!(res = ?res);

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
