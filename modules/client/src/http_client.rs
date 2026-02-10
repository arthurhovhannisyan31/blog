use common::constants::{QUERY_LIMIT, QUERY_OFFSET, http_route, http_scope};
use proto_generator::blog::{
  AuthRequest, AuthResponse, CreatePostRequest, CreateUserRequest,
  DeletePostRequest, ListPostResponse, PostResponse, UpdatePostRequest,
};
use reqwest::{Client, header::AUTHORIZATION};

use crate::AbstractBlogClient;
use crate::error::BlogClientError;

pub struct HttpBlogClient {
  pub client: Client,
  pub base_url: String,
}

impl HttpBlogClient {
  pub fn new(client: Client, base_url: String) -> Self {
    Self { client, base_url }
  }
}

impl AbstractBlogClient for HttpBlogClient {
  async fn register(
    &mut self,
    username: String,
    email: String,
    password: String,
  ) -> Result<AuthResponse, BlogClientError> {
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::REGISTER.trim_start_matches('/')
    );

    let resp = self
      .client
      .post(url)
      .json(&CreateUserRequest {
        email,
        password,
        username,
      })
      .send()
      .await?;

    match resp.error_for_status() {
      Ok(res) => {
        let data = res.json::<AuthResponse>().await?;

        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn login(
    &mut self,
    email: String,
    password: String,
  ) -> Result<AuthResponse, BlogClientError> {
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::LOGIN.trim_start_matches('/')
    );
    let resp = self
      .client
      .post(url)
      .json(&AuthRequest { email, password })
      .send()
      .await?;

    match resp.error_for_status() {
      Ok(res) => {
        let data = res.json::<AuthResponse>().await?;

        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn create_post(
    &mut self,
    token: &str,
    title: String,
    content: String,
  ) -> Result<PostResponse, BlogClientError> {
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self
      .client
      .post(url)
      .header(AUTHORIZATION, token)
      .json(&CreatePostRequest { content, title })
      .send()
      .await?;
    match resp.error_for_status() {
      Ok(res) => {
        let data = res.json::<PostResponse>().await?;

        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn get_post(
    &mut self,
    id: i64,
  ) -> Result<PostResponse, BlogClientError> {
    let url = format!(
      "{}/{}/{}/{id}",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self.client.get(url).send().await?;

    match resp.error_for_status() {
      Ok(res) => {
        let data = res.json::<PostResponse>().await?;

        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn list_posts(
    &mut self,
    limit: Option<u64>,
    offset: Option<u64>,
  ) -> Result<ListPostResponse, BlogClientError> {
    let limit = limit.unwrap_or(QUERY_LIMIT);
    let offset = offset.unwrap_or(QUERY_OFFSET);
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self
      .client
      .get(url)
      .query(&[("limit", limit), ("offset", offset)])
      .send()
      .await?;

    match resp.error_for_status() {
      Ok(res) => {
        let data = res.json::<ListPostResponse>().await?;

        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn update_post(
    &mut self,
    token: &str,
    id: i64,
    title: Option<String>,
    content: Option<String>,
  ) -> Result<PostResponse, BlogClientError> {
    let url = format!(
      "{}/{}/{}/{id}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self
      .client
      .put(url)
      .header(AUTHORIZATION, token)
      .json(&UpdatePostRequest { id, content, title })
      .send()
      .await?;

    match resp.error_for_status() {
      Ok(res) => {
        let data = res.json::<PostResponse>().await?;

        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn delete_post(
    &mut self,
    token: &str,
    id: i64,
  ) -> Result<(), BlogClientError> {
    let url = format!(
      "{}/{}/{}/{id}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self
      .client
      .delete(url)
      .header(AUTHORIZATION, token)
      .json(&DeletePostRequest { id })
      .send()
      .await?;

    match resp.error_for_status() {
      Ok(_) => Ok(()),
      Err(err) => Err(err.into()),
    }
  }
}
