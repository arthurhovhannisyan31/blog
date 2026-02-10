use blog_client::{AbstractBlogClient, grpc_client::GrpcBlogClient};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt().with_env_filter("info").init();

  let mut client =
    GrpcBlogClient::new("http://127.0.0.1:50051".to_string()).await?;

  let list_posts_response = client.list_posts(None, None).await?;

  info!("List existing posts \n\n");
  for post in list_posts_response.posts {
    info!(post = ?post, "Post: \n\n");
  }

  let token: String;

  // Make example code logic idempotent
  if let Ok(login_response) =
    client.login("email".into(), "password".into()).await
  {
    info!(auth = ?login_response, "Login user: \n\n");

    token = format!("Bearer {}", login_response.token);
  } else {
    let register_response = client
      .register("username".into(), "email".into(), "password".into())
      .await?;
    info!(auth = ?register_response, "Registered user: \n\n");

    token = format!("Bearer {}", register_response.token);
  }

  let create_post_response = client
    .create_post(&token, "title".into(), "content".into())
    .await?;
  info!(post = ?create_post_response, "Create post: \n\n");

  let get_post_response = client.get_post(create_post_response.id).await?;
  info!(post = ?get_post_response, "Read created post: \n\n");

  let update_post_response = client
    .update_post(
      &token,
      create_post_response.id,
      None,
      Some("new_content".into()),
    )
    .await?;
  info!(post = ?update_post_response, "Update created post: \n\n");

  let delete_post_response =
    client.delete_post(&token, create_post_response.id).await?;
  info!(post = ?delete_post_response, "Delete created post: \n\n");

  Ok(())
}
