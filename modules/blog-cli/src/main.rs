use blog_client::AbstractBlogClient;
use clap::Parser;
use tracing::info;

mod configs;
mod error;
mod init_client;
mod logging;
mod token;

use configs::{Cli, Commands};
use error::CliError;
use init_client::init_client;
use logging::init_logging;
use token::{BLOG_TOKEN_PATH, read_token, save_token};

#[tokio::main]
async fn main() -> Result<(), CliError> {
  init_logging();

  let Cli {
    command,
    grpc,
    server,
  } = Cli::parse();

  let mut client = init_client(grpc, server).await?;
  let token = read_token(BLOG_TOKEN_PATH.to_string()).await?;
  client.set_token(token);

  match command {
    Commands::Register {
      email,
      password,
      username,
    } => {
      let response = client.register(username, email, password).await?;
      info!(auth = ?response, "Registered user: ");

      let token = format!("Bearer {}", response.token);
      client.set_token(token.clone());
      save_token(BLOG_TOKEN_PATH.to_string(), token).await?;
    }
    Commands::Login { email, password } => {
      let response = client.login(email, password).await?;
      info!(auth = ?response, "Login user: ");

      let token = format!("Bearer {}", response.token);
      client.set_token(token.clone());
      save_token(BLOG_TOKEN_PATH.to_string(), token).await?;
    }
    Commands::Create { content, title } => {
      let response = client.create_post("", title, content).await?;

      info!(post = ?response, "Post created: ");
    }
    Commands::Get { id } => {
      let response = client.get_post(id as i64).await?;

      info!(post = ?response, "Get post: ");
    }
    Commands::Update { id, content, title } => {
      let response = client.update_post("", id as i64, title, content).await?;

      info!(post = ?response, "Post updated: ");
    }
    Commands::Delete { id } => {
      let _ = client.delete_post("", id as i64).await?;

      info!(id = id, "Post deleted: ");
    }
    Commands::List { limit, offset } => {
      let response = client.list_posts(Some(limit), Some(offset)).await?;

      info!(
        limit = response.limit,
        offset = response.offset,
        "Posts list response"
      );
      for post in response.posts {
        info!(post = ?post, "Post: \n\n");
      }
    }
  }

  Ok(())
}
