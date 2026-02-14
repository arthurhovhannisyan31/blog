use crate::error::BlogClientError;
use crate::grpc_client::GrpcBlogClient;
use crate::http_client::HttpBlogClient;
use crate::{AbstractBlogClient, Transport};
use proto_generator::blog::{AuthResponse, PostResponse, PostsListResponse};
use reqwest::Client;

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
  pub fn set_token(&mut self, token: String) {
    self.token = Some(token);
  }
  pub fn get_token(&self) -> String {
    self.token.clone().unwrap_or_default()
  }
}

impl AbstractBlogClient for BlogClient {
  async fn register(
    &mut self,
    username: String,
    email: String,
    password: String,
  ) -> Result<AuthResponse, BlogClientError> {
    match self.transport {
      Transport::Grpc(_) => {
        if let Some(client) = &mut self.grpc_client {
          let response = client.register(username, email, password).await?;
          self.set_token(response.token.clone());

          return Ok(response);
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
      Transport::Http(_) => {
        if let Some(client) = &mut self.http_client {
          let response = client.register(username, email, password).await?;
          self.set_token(response.token.clone());

          return Ok(response);
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
    }
  }
  async fn login(
    &mut self,
    username: String,
    password: String,
  ) -> Result<AuthResponse, BlogClientError> {
    match self.transport {
      Transport::Grpc(_) => {
        if let Some(client) = &mut self.grpc_client {
          let response = client.login(username, password).await?;
          self.set_token(response.token.clone());

          return Ok(response);
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
      Transport::Http(_) => {
        if let Some(client) = &mut self.http_client {
          let response = client.login(username, password).await?;
          self.set_token(response.token.clone());

          return Ok(response);
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
    }
  }
  async fn create_post(
    &mut self,
    _token: &str,
    title: String,
    content: String,
  ) -> Result<PostResponse, BlogClientError> {
    let token = &self.get_token();

    match self.transport {
      Transport::Grpc(_) => {
        if let Some(client) = &mut self.grpc_client {
          return client.create_post(token, title, content).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
      Transport::Http(_) => {
        if let Some(client) = &mut self.http_client {
          return client.create_post(token, title, content).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
    }
  }
  async fn get_post(
    &mut self,
    id: i64,
  ) -> Result<PostResponse, BlogClientError> {
    match self.transport {
      Transport::Grpc(_) => {
        if let Some(client) = &mut self.grpc_client {
          return client.get_post(id).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
      Transport::Http(_) => {
        if let Some(client) = &mut self.http_client {
          return client.get_post(id).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
    }
  }
  async fn list_posts(
    &mut self,
    limit: Option<u64>,
    offset: Option<u64>,
  ) -> Result<PostsListResponse, BlogClientError> {
    match self.transport {
      Transport::Grpc(_) => {
        if let Some(client) = &mut self.grpc_client {
          return client.list_posts(limit, offset).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
      Transport::Http(_) => {
        if let Some(client) = &mut self.http_client {
          return client.list_posts(limit, offset).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
    }
  }
  async fn update_post(
    &mut self,
    _token: &str,
    id: i64,
    title: Option<String>,
    content: Option<String>,
  ) -> Result<PostResponse, BlogClientError> {
    let token = &self.get_token();

    match self.transport {
      Transport::Grpc(_) => {
        if let Some(client) = &mut self.grpc_client {
          return client.update_post(token, id, title, content).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
      Transport::Http(_) => {
        if let Some(client) = &mut self.http_client {
          return client.update_post(token, id, title, content).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
    }
  }
  async fn delete_post(
    &mut self,
    _token: &str,
    id: i64,
  ) -> Result<(), BlogClientError> {
    let token = &self.get_token();

    match self.transport {
      Transport::Grpc(_) => {
        if let Some(client) = &mut self.grpc_client {
          return client.delete_post(token, id).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
      Transport::Http(_) => {
        if let Some(client) = &mut self.http_client {
          return client.delete_post(token, id).await;
        }
        Err(BlogClientError::Internal("Missing grpc client".into()))
      }
    }
  }
}
