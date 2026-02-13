use common::constants::{http_route, http_scope};
use reqwest::header::AUTHORIZATION;
use reqwest::Client;

use crate::infrastructure::model::{
  AuthRequest, AuthResponse, CreateUserRequest, PostResponse,
  PostsListResponse, UpdatePostRequest,
};

#[derive(Clone, Debug)]
pub struct BlogClient {
  client: Client,
  base_url: String,
}

impl BlogClient {
  pub fn new(base_url: String) -> anyhow::Result<Self> {
    let client = Client::builder()
      .user_agent("user-Agent: wasm-fe")
      .build()?;

    Ok(Self { client, base_url })
  }
  pub async fn register(
    &self,
    username: String,
    email: String,
    password: String,
  ) -> anyhow::Result<AuthResponse> {
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::REGISTER.trim_start_matches('/')
    );
    let response = self
      .client
      .post(url)
      .json(&CreateUserRequest {
        username,
        email,
        password,
      })
      .send()
      .await?;
    let auth = response.json::<AuthResponse>().await?;

    Ok(auth)
  }
  pub async fn login(
    &self,
    email: String,
    password: String,
  ) -> anyhow::Result<AuthResponse> {
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::LOGIN.trim_start_matches('/')
    );
    let response = self
      .client
      .post(url)
      .json(&AuthRequest { email, password })
      .send()
      .await?;
    let auth = response.json::<AuthResponse>().await?;

    Ok(auth)
  }
  pub async fn create_post(
    &self,
    token: String,
    title: String,
    content: String,
  ) -> anyhow::Result<PostResponse> {
    let url = format!(
      "{}/{}/{}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let response = self
      .client
      .post(url)
      .header(AUTHORIZATION, token)
      .json(&UpdatePostRequest {
        title: Some(title),
        content: Some(content),
      })
      .send()
      .await?;
    let post = response.json::<PostResponse>().await?;

    Ok(post)
  }
  pub async fn get_post(&self, id: i64) -> anyhow::Result<PostResponse> {
    let url = format!(
      "{}/{}/{}/{id}",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let response = self.client.get(url).send().await?;
    let post = response.json::<PostResponse>().await?;

    Ok(post)
  }
  pub async fn list_posts(&self) -> anyhow::Result<PostsListResponse> {
    let url = format!(
      "{}/{}/{}?limit=50&offset=0",
      self.base_url.trim_end_matches('/'),
      http_scope::PUBLIC.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let response = self.client.get(url).send().await?;
    let posts_list = response.json::<PostsListResponse>().await?;

    Ok(posts_list)
  }
  pub async fn update_post(
    &self,
    token: String,
    id: i64,
    title: String,
    content: String,
  ) -> anyhow::Result<PostResponse> {
    let url = format!(
      "{}/{}/{}/{id}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let response = self
      .client
      .put(url)
      .header(AUTHORIZATION, token)
      .json(&UpdatePostRequest {
        title: Some(title),
        content: Some(content),
      })
      .send()
      .await?;
    let post = response.json::<PostResponse>().await?;

    Ok(post)
  }
  pub async fn delete_post(
    &self,
    token: String,
    id: i64,
  ) -> anyhow::Result<()> {
    let url = format!(
      "{}/{}/{}/{id}",
      self.base_url.trim_end_matches('/'),
      http_scope::PROTECTED.trim_start_matches('/'),
      http_route::POST.trim_start_matches('/')
    );
    let _ = self
      .client
      .delete(url)
      .header(AUTHORIZATION, token)
      .send()
      .await?;

    Ok(())
  }
}
