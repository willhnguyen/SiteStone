use crate::domain::bookmark::Bookmark;
use crate::error::AppError;

pub trait BookmarkRepository: Send + Sync {
  async fn create(&self, bookmark: &Bookmark) -> Result<(), AppError>;
  async fn get_by_id(&self, id: &str) -> Result<Option<Bookmark>, AppError>;
}
