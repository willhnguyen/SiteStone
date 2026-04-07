use crate::domain::bookmark::{Bookmark, BookmarkCategory, BookmarkStatus};
use crate::error::AppError;

#[derive(Debug, Default, Clone)]
pub struct BookmarkFilter {
  pub status: Option<BookmarkStatus>,
  pub category: Option<BookmarkCategory>,
  pub include_deleted: bool,
  pub limit: Option<i64>,
  pub offset: Option<i64>,
}

pub trait BookmarkRepository: Send + Sync {
  async fn create(&self, bookmark: &Bookmark) -> Result<(), AppError>;
  async fn get_by_id(&self, id: &str) -> Result<Option<Bookmark>, AppError>;
  async fn list(&self, filter: &BookmarkFilter) -> Result<Vec<Bookmark>, AppError>;
  async fn update(&self, bookmark: &Bookmark) -> Result<(), AppError>;
  async fn soft_delete(&self, id: &str, deleted_at: &str) -> Result<(), AppError>;
}
