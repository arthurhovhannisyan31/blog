use std::cmp::min;

use crate::application::{blog_service::BlogService, error::ApplicationError};
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::post::Post;
use crate::presentation::http::dto::{
  AuthenticatedUser, CreatePostRequest, GetPostsQueryParams, ListPostResponse,
  PostResponse, UpdatePostRequest,
};
use actix_web::{HttpResponse, delete, get, post, put, web};
use common::constants::{QUERY_LIMIT, QUERY_LIMIT_STEP, QUERY_OFFSET};
use common::utils::get_next_pagination;
use tracing::info;

pub fn ensure_owner(
  post: &Post,
  user: &AuthenticatedUser,
) -> Result<(), ApplicationError> {
  if post.author_id != user.user_id {
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
    .create_post(payload.title.clone(), payload.content.clone(), user.user_id)
    .await?;

  info!(
    user_id = %user.user_id,
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
  let post = blog_service.get_post(id).await?;

  Ok(HttpResponse::Ok().json(post))
}

#[get("/posts")]
pub async fn list_posts(
  blog_service: web::Data<BlogService<PostgresPostRepository>>,
  query_params: web::Query<GetPostsQueryParams>,
) -> Result<HttpResponse, ApplicationError> {
  let params = query_params.into_inner();
  let limit = params.limit.unwrap_or(QUERY_LIMIT);
  let offset = params.offset.unwrap_or(QUERY_OFFSET);

  let total = blog_service.get_posts_count().await? as u64;
  let posts = blog_service
    .list_posts(limit as i64, offset as i64)
    .await?
    .into_iter()
    .map(PostResponse::from)
    .collect();

  let (next_offset, next_limit) = get_next_pagination(total, limit);

  Ok(HttpResponse::Ok().json(ListPostResponse {
    posts,
    total,
    limit: next_limit,
    offset: next_offset,
  }))
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
      user.user_id,
    )
    .await?;

  info!(
    user_id = %user.user_id,
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
