use crate::domain::bookmark::Bookmark;
use crate::error::AppError;

pub trait BookmarkRepository: Send + Sync {
  async fn create(&self, bookmark: &Bookmark) -> Result<(), AppError>;
}
