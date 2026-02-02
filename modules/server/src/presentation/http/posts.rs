use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use tracing::info;

use crate::application::{blog_service::BlogService, error::ApplicationError};
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::post::Post;
use crate::presentation::http::dto::GetPostsQueryParams;
use crate::presentation::{
  auth::AuthenticatedUser,
  http::dto::{CreatePostRequest, UpdatePostRequest},
};

pub fn configure_protected(cfg: &mut web::ServiceConfig) {
  cfg.service(create_post).service(update_post);
}

pub fn configure_public(cfg: &mut web::ServiceConfig) {
  cfg.service(get_post).service(get_posts);
}

fn ensure_owner(
  account: &Post,
  user: &AuthenticatedUser,
) -> Result<(), ApplicationError> {
  if account.id != user.id {
    Err(ApplicationError::Forbidden)
  } else {
    Ok(())
  }
}

#[post("/posts")]
async fn create_post(
  blog_service: web::Data<BlogService<PostgresPostRepository>>,
  payload: web::Json<CreatePostRequest>,
  user: AuthenticatedUser,
) -> Result<HttpResponse, ApplicationError> {
  let post = blog_service
    .create_post(payload.title.clone(), payload.content.clone(), user.id)
    .await?;

  info!(
    user_id = %user.id,
    title = %payload.title,
    content = %payload.content,
    "Post created: "
  );

  Ok(HttpResponse::Created().json(post))
}

#[get("/posts/{id}")]
async fn get_post(
  blog_service: web::Data<BlogService<PostgresPostRepository>>,
  path: web::Path<i64>,
) -> Result<HttpResponse, ApplicationError> {
  let id = path.into_inner();
  let post_data = blog_service.get_post(id).await?;

  Ok(HttpResponse::Ok().json(post_data))
}

#[put("/posts/{id}")]
async fn update_post(
  blog_service: web::Data<BlogService<PostgresPostRepository>>,
  path: web::Path<i64>,
  user: AuthenticatedUser,
  payload: web::Json<UpdatePostRequest>,
) -> Result<HttpResponse, ApplicationError> {
  let id = path.into_inner();
  let post = blog_service.get_post(id).await?;

  ensure_owner(&post, &user)?;

  let update_post_data = payload.into_inner();
  // TODO Ensure 404 returned
  let updated_post = blog_service
    .update_post(
      id,
      update_post_data.title,
      update_post_data.content,
      user.id,
    )
    .await?;

  info!(
    user_id = %user.id,
    post_id = post.id,
    "Post updated"
  );

  Ok(HttpResponse::Ok().json(updated_post))
}

#[delete("/posts/{id}")]
async fn delete_post(
  blog_service: web::Data<BlogService<PostgresPostRepository>>,
  path: web::Path<i64>,
  user: AuthenticatedUser,
) -> Result<HttpResponse, ApplicationError> {
  let id = path.into_inner();
  let post = blog_service.get_post(id).await?;

  ensure_owner(&post, &user)?;

  blog_service.delete_post(id).await?;

  Ok(HttpResponse::NoContent().finish())
}

#[get("/posts")]
async fn get_posts(
  blog_service: web::Data<BlogService<PostgresPostRepository>>,
  query_params: web::Query<GetPostsQueryParams>,
) -> Result<HttpResponse, ApplicationError> {
  let params = query_params.into_inner();
  let limit = params.limit.unwrap_or(10);
  let offset = params.offset.unwrap_or(0);
  let total = blog_service.get_posts_count().await?;
  let posts = blog_service.get_posts(limit, offset).await?;
  let next_limit = total.saturating_sub(limit);

  Ok(HttpResponse::Ok().json(json!({
    "posts": posts,
    "total": total,
    "limit": next_limit,
    "offset": limit,
  })))
}
