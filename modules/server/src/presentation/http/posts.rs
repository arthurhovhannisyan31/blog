use actix_web::cookie::time::macros::offset;
use actix_web::{delete, get, post, put, web, HttpResponse, Scope};
use serde_json::json;
use std::cmp::min;
use tracing::info;

use crate::application::{blog_service::BlogService, error::ApplicationError};
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::post::Post;
use crate::presentation::http::constants::{
  QUERY_LIMIT, QUERY_LIMIT_STEP, QUERY_OFFSET,
};
use crate::presentation::{
  auth::AuthenticatedUser,
  http::dto::{CreatePostRequest, GetPostsQueryParams, UpdatePostRequest},
};

pub fn ensure_owner(
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
pub async fn create_post(
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
pub async fn get_post(
  blog_service: web::Data<BlogService<PostgresPostRepository>>,
  path: web::Path<i64>,
) -> Result<HttpResponse, ApplicationError> {
  let id = path.into_inner();
  let post_data = blog_service.get_post(id).await?;

  Ok(HttpResponse::Ok().json(post_data))
}

#[put("/posts/{id}")]
pub async fn update_post(
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
pub async fn delete_post(
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
pub async fn get_posts(
  blog_service: web::Data<BlogService<PostgresPostRepository>>,
  query_params: web::Query<GetPostsQueryParams>,
) -> Result<HttpResponse, ApplicationError> {
  let params = query_params.into_inner();
  let limit = params.limit.unwrap_or(QUERY_LIMIT);
  let offset = params.offset.unwrap_or(QUERY_OFFSET);

  // TODO Che how negative numbers will be treated

  // if limit < 0 || offset < 0 {
  //   return Err(ApplicationError::Validation(
  //     "Limit and offset must be positive values".to_string(),
  //   ));
  // }

  let total = blog_service.get_posts_count().await? as u64;

  // TODO Check if range inclusive
  let next_offset = min(total, limit);
  let next_limit = next_offset + QUERY_LIMIT_STEP;
  let next_limit = if next_limit > total { 0 } else { next_limit };
  let posts = blog_service.get_posts(limit as i64, offset as i64).await?;

  Ok(HttpResponse::Ok().json(json!({
    "posts": posts,
    "total": total,
    "limit": next_limit,
    "offset": next_offset,
  })))
}
