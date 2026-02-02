use crate::application::error::ApplicationError;
use crate::data::post_repository::PostRepository;
use crate::domain::post::Post;
use std::sync::Arc;

pub struct BlogPostsService<R: PostRepository + 'static> {
  repo: Arc<R>,
}

impl<R> BlogPostsService<R>
where
  R: PostRepository + 'static,
{
  pub fn new(repo: Arc<R>) -> Self {
    Self { repo }
  }

  // TODO Check if it works
  // #[instrument(skip(self))]
  pub async fn create_post(
    &self,
    title: String,
    content: Option<String>,
    author_id: i64,
  ) -> Result<Post, ApplicationError> {
    let post = Post::new(title, content.unwrap_or("".into()), author_id)
      .map_err(ApplicationError::from)?;

    self.repo.create(post).await.map_err(ApplicationError::from)
  }

  // #[instrument(skip(self))]
  pub async fn get_post(
    &self,
    id: i64,
  ) -> Result<Option<Post>, ApplicationError> {
    self.repo.get(id).await.map_err(ApplicationError::from)
  }

  // #[instrument(skip(self))]
  pub async fn get_all_posts(
    &self,
    author_id: i64,
  ) -> Result<Vec<Post>, ApplicationError> {
    self
      .repo
      .get_all(author_id)
      .await
      .map_err(ApplicationError::from)
  }

  // #[instrument(skip(self))]
  pub async fn update_post(
    &self,
    title: String,
    content: String,
    author_id: i64,
  ) -> Result<Post, ApplicationError> {
    let post =
      Post::new(title, content, author_id).map_err(ApplicationError::from)?;

    self.repo.update(post).await.map_err(ApplicationError::from)
  }

  // #[instrument(skip(self))]
  pub async fn delete_post(&self, id: i64) -> Result<(), ApplicationError> {
    self.repo.delete(id).await.map_err(ApplicationError::from)
  }
}
