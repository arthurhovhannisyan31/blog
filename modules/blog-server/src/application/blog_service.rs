use crate::application::error::ApplicationError;
use crate::data::post_repository::PostRepository;
use crate::domain::post::Post;

#[derive(Clone)]
pub struct BlogService<R: PostRepository + 'static> {
  repo: R,
}

impl<R> BlogService<R>
where
  R: PostRepository + 'static,
{
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn create_post(
    &self,
    title: String,
    content: String,
    author_id: i64,
  ) -> Result<Post, ApplicationError> {
    let post =
      Post::new(title, content, author_id).map_err(ApplicationError::from)?;

    self.repo.create(post).await.map_err(ApplicationError::from)
  }

  pub async fn get_post(&self, id: i64) -> Result<Post, ApplicationError> {
    match self.repo.get(id).await.map_err(ApplicationError::from)? {
      Some(post) => Ok(post),
      None => Err(ApplicationError::NotFound(format!("Post {}", id))),
    }
  }

  pub async fn get_posts_count(&self) -> Result<i64, ApplicationError> {
    let count = self.repo.get_row_count().await?;

    Ok(count)
  }

  pub async fn list_posts(
    &self,
    limit: i64,
    offset: i64,
  ) -> Result<Vec<Post>, ApplicationError> {
    self
      .repo
      .list(limit, offset)
      .await
      .map_err(ApplicationError::from)
  }

  pub async fn update_post(
    &self,
    id: i64,
    title: String,
    content: String,
    author_id: i64,
  ) -> Result<Post, ApplicationError> {
    let post =
      Post::new(title, content, author_id).map_err(ApplicationError::from)?;

    self
      .repo
      .update(id, post)
      .await
      .map_err(ApplicationError::from)
  }

  pub async fn delete_post(&self, id: i64) -> Result<(), ApplicationError> {
    self.repo.delete(id).await.map_err(ApplicationError::from)
  }
}
