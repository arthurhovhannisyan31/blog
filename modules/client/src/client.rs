use proto_generator::blog::{AuthResponse, ListPostResponse, PostResponse};
use reqwest::Client;

use crate::error::BlogClientError;
use crate::grpc_client::GrpcBlogClient;
use crate::http_client::HttpBlogClient;
use crate::{AbstractBlogClient, Transport};

pub struct BlogClient {
  transport: Transport,
  http_client: Option<HttpBlogClient>,
  grpc_client: Option<GrpcBlogClient>,
  token: Option<String>,
}

impl BlogClient {
  pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
    match transport.clone() {
      Transport::Grpc(addr) => {
        let client = GrpcBlogClient::new(addr).await.map_err(|e| {
          BlogClientError::Internal(format!("Failed builing grpc client {e:?}"))
        })?;

        Ok(Self {
          transport,
          http_client: None,
          grpc_client: Some(client),
          token: None,
        })
      }
      Transport::Http(addr) => {
        let request_client = Client::builder().build().map_err(|e| {
          BlogClientError::Internal(format!("Failed builing grpc client {e:?}"))
        })?;

        Ok(Self {
          transport,
          http_client: Some(HttpBlogClient::new(request_client, addr)),
          grpc_client: None,
          token: None,
        })
      }
    }
  }
  fn set_token(&mut self, token: String) {
    self.token = Some(format!("Bearer {}", token));
  }
  fn get_token(&self) -> String {
    self.token.clone().unwrap_or_default()
  }
}

impl AbstractBlogClient for BlogClient {
  fn register(
    &mut self,
    username: String,
    email: String,
    password: String,
  ) -> impl Future<Output = Result<AuthResponse, BlogClientError>> {
    match self.transport {
      Transport::Grpc(_) => {
        if let Some(mut client) = self.grpc_client {
          client.register(username, email, password)
        } else {
          Err(BlogClientError::Transport("Missing grpc client".into()))
        }
      }
      Transport::Http(_) => {}
    }
  }
  fn login(
    &mut self,
    username: String,
    password: String,
  ) -> impl Future<Output = Result<AuthResponse, BlogClientError>> {
    match self.transport {
      Transport::Grpc(_) => {}
      Transport::Http(_) => {}
    }
  }
  fn create_post(
    &mut self,
    token: &str,
    title: String,
    content: String,
  ) -> impl Future<Output = Result<PostResponse, BlogClientError>> {
    match self.transport {
      Transport::Grpc(_) => {}
      Transport::Http(_) => {}
    }
  }
  fn get_post(
    &mut self,
    id: i64,
  ) -> impl Future<Output = Result<PostResponse, BlogClientError>> {
    match self.transport {
      Transport::Grpc(_) => {}
      Transport::Http(_) => {}
    }
  }
  fn list_posts(
    &mut self,
    limit: Option<u64>,
    offset: Option<u64>,
  ) -> impl Future<Output = Result<ListPostResponse, BlogClientError>> {
    match self.transport {
      Transport::Grpc(_) => {}
      Transport::Http(_) => {}
    }
  }
  fn update_post(
    &mut self,
    token: &str,
    id: i64,
    title: String,
    content: String,
  ) -> impl Future<Output = Result<PostResponse, BlogClientError>> {
    match self.transport {
      Transport::Grpc(_) => {}
      Transport::Http(_) => {}
    }
  }
  fn delete_post(
    &mut self,
    token: &str,
    id: i64,
  ) -> impl Future<Output = Result<(), BlogClientError>> {
    match self.transport {
      Transport::Grpc(_) => {}
      Transport::Http(_) => {}
    }
  }
}
