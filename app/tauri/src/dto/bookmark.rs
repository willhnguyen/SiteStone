use crate::domain::bookmark::{Bookmark, BookmarkCategory, BookmarkStatus};
use serde::{Deserialize, Serialize};

/// Bookmark as sent over Tauri IPC to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkDto {
  pub id: String,
  pub url: String,
  pub url_normalized: String,
  pub title: String,
  pub description: String,
  pub tags: Vec<String>,
  pub notes: String,
  pub status: BookmarkStatus,
  pub category: BookmarkCategory,
  pub character_count: Option<i64>,
  pub browser_bookmark_id: Option<String>,
  pub deleted_at: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

impl From<Bookmark> for BookmarkDto {
  fn from(b: Bookmark) -> Self {
    Self {
      id: b.id,
      url: b.url,
      url_normalized: b.url_normalized,
      title: b.title,
      description: b.description,
      tags: b.tags,
      notes: b.notes,
      status: b.status,
      category: b.category,
      character_count: b.character_count,
      browser_bookmark_id: b.browser_bookmark_id,
      deleted_at: b.deleted_at,
      created_at: b.created_at,
      updated_at: b.updated_at,
    }
  }
}

/// Request payload to update an existing bookmark.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookmarkRequest {
  pub title: Option<String>,
  pub description: Option<String>,
  pub tags: Option<Vec<String>>,
  pub notes: Option<String>,
  pub status: Option<BookmarkStatus>,
  pub category: Option<BookmarkCategory>,
}

/// Query parameters for listing bookmarks.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListBookmarksRequest {
  pub status: Option<BookmarkStatus>,
  pub category: Option<BookmarkCategory>,
  pub include_deleted: Option<bool>,
  pub limit: Option<i64>,
  pub offset: Option<i64>,
}

/// Request payload to create a new bookmark.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookmarkRequest {
  pub url: String,
  pub title: Option<String>,
  pub description: Option<String>,
  pub tags: Option<Vec<String>>,
  pub notes: Option<String>,
  pub browser_bookmark_id: Option<String>,
}
