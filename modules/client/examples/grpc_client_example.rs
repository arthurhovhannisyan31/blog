use client::{AbstractBlogClient, grpc_client::GrpcBlogClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt().with_env_filter("info").init();

  let mut client =
    GrpcBlogClient::new("http://127.0.0.1:50051".to_string()).await?;

  let mut stream = client.list_posts(None, None).await?;

  //

  // try to run all api calls here

  Ok(())
}
