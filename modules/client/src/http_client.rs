use std::error::Error;

use common::constants::{http_route, http_scope};
use proto_generator::blog::{
  AuthRequest, AuthResponse, CreatePostRequest, CreateUserRequest,
  DeletePostRequest, PostResponse, UpdatePostRequest,
};
use reqwest::Client;
use reqwest::header::AUTHORIZATION;

use crate::client::BlogClientImpl;

pub struct HttpBlogClient {
  pub client: Client,
  pub base_url: String,
  pub token: Option<String>,
}

impl HttpBlogClient {
  fn set_token(&mut self, token: String) {
    self.token = Some(format!("Bearer {}", token));
  }
  fn new(client: Client, base_url: String) -> Self {
    Self {
      client,
      base_url,
      token: None,
    }
  }
}

impl BlogClientImpl<Vec<PostResponse>, ()> for HttpBlogClient {
  async fn register(
    &mut self,
    username: String,
    email: String,
    password: String,
  ) -> Result<AuthResponse, Box<dyn Error>> {
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
        self.set_token(data.token.clone());
        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn login(
    &mut self,
    email: String,
    password: String,
  ) -> Result<AuthResponse, Box<dyn Error>> {
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
        self.set_token(data.token.clone());
        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn create_post(
    &mut self,
    title: String,
    content: String,
  ) -> Result<PostResponse, Box<dyn Error>> {
    let token_value = self.token.clone().unwrap_or_default();
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self
      .client
      .post(url)
      .header(AUTHORIZATION, token_value)
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
  ) -> Result<PostResponse, Box<dyn Error>> {
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
    limit: Option<i64>,
    offset: Option<i64>,
  ) -> Result<Vec<PostResponse>, Box<dyn Error>> {
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self.client.get(url).send().await?;

    match resp.error_for_status() {
      Ok(res) => {
        let data = res.json::<Vec<PostResponse>>().await?;

        Ok(data)
      }
      Err(err) => Err(err.into()),
    }
  }
  async fn update_post(
    &mut self,
    id: i64,
    title: String,
    content: String,
  ) -> Result<PostResponse, Box<dyn Error>> {
    let token_value = self.token.clone().unwrap_or_default();
    let url = format!(
      "{}/{}/{}/{id}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self
      .client
      .put(url)
      .header(AUTHORIZATION, token_value)
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
  async fn delete_post(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
    let token_value = self.token.clone().unwrap_or_default();
    let url = format!(
      "{}/{}/{}/{id}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let resp = self
      .client
      .delete(url)
      .header(AUTHORIZATION, token_value)
      .json(&DeletePostRequest { id })
      .send()
      .await?;

    match resp.error_for_status() {
      Ok(_) => Ok(()),
      Err(err) => Err(err.into()),
    }
  }
}
