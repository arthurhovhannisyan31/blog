use client::{client::BlogClientImpl, http_client::HttpBlogClient};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt().with_env_filter("info").init();

  let client = Client::builder()
    .build()
    .expect("failed to build http client");

  let mut blog_client =
    HttpBlogClient::new(client, "http://127.0.0.1:8080/api".to_string());

  let posts = blog_client.list_posts(None, None).await?;

  // for post in posts {
  //   info!(post = ?post, "Post: \n\n");
  // }

  // try to run all api calls here

  Ok(())
}
