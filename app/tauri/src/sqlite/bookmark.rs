use crate::domain::bookmark::{Bookmark, BookmarkCategory, BookmarkStatus};
use crate::error::AppError;
use crate::repository::bookmark::BookmarkRepository;
use sqlx::sqlite::SqlitePool;
use std::str::FromStr;

#[derive(sqlx::FromRow)]
pub(super) struct BookmarkRow {
  pub id: String,
  pub url: String,
  pub url_normalized: String,
  pub title: String,
  pub description: String,
  pub tags: String,
  pub notes: String,
  pub status: String,
  pub category: String,
  pub character_count: Option<i64>,
  pub browser_bookmark_id: Option<String>,
  pub deleted_at: Option<String>,
  pub created_at: String,
  pub updated_at: String,
}

impl TryFrom<BookmarkRow> for Bookmark {
  type Error = AppError;

  fn try_from(row: BookmarkRow) -> Result<Self, Self::Error> {
    let tags: Vec<String> = serde_json::from_str(&row.tags)?;
    let status = BookmarkStatus::from_str(&row.status).map_err(AppError::InvalidInput)?;
    let category = BookmarkCategory::from_str(&row.category).map_err(AppError::InvalidInput)?;
    Ok(Bookmark {
      id: row.id,
      url: row.url,
      url_normalized: row.url_normalized,
      title: row.title,
      description: row.description,
      tags,
      notes: row.notes,
      status,
      category,
      character_count: row.character_count,
      browser_bookmark_id: row.browser_bookmark_id,
      deleted_at: row.deleted_at,
      created_at: row.created_at,
      updated_at: row.updated_at,
    })
  }
}

#[derive(Clone)]
pub struct SqliteBookmarkRepository {
  pool: SqlitePool,
}

impl SqliteBookmarkRepository {
  pub fn new(pool: SqlitePool) -> Self {
    Self { pool }
  }
}

impl BookmarkRepository for SqliteBookmarkRepository {
  async fn get_by_id(&self, id: &str) -> Result<Option<Bookmark>, AppError> {
    let row = sqlx::query_as::<_, BookmarkRow>(
      "SELECT id, url, url_normalized, title, description, tags, notes,
              status, category, character_count, browser_bookmark_id,
              deleted_at, created_at, updated_at
       FROM bookmarks WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&self.pool)
    .await?;
    row.map(Bookmark::try_from).transpose()
  }

  async fn create(&self, b: &Bookmark) -> Result<(), AppError> {
    let tags = serde_json::to_string(&b.tags)?;
    let result = sqlx::query(
      "INSERT INTO bookmarks
        (id, url, url_normalized, title, description, tags, notes,
         status, category, character_count, browser_bookmark_id,
         deleted_at, created_at, updated_at)
       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&b.id)
    .bind(&b.url)
    .bind(&b.url_normalized)
    .bind(&b.title)
    .bind(&b.description)
    .bind(&tags)
    .bind(&b.notes)
    .bind(b.status.as_str())
    .bind(b.category.as_str())
    .bind(b.character_count)
    .bind(&b.browser_bookmark_id)
    .bind(&b.deleted_at)
    .bind(&b.created_at)
    .bind(&b.updated_at)
    .execute(&self.pool)
    .await;

    match result {
      Ok(_) => Ok(()),
      Err(e) => {
        let is_unique = matches!(&e, sqlx::Error::Database(db) if db.is_unique_violation());
        if is_unique {
          Err(AppError::Duplicate(b.url_normalized.clone()))
        } else {
          Err(AppError::Database(e))
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::sqlite::db;

  async fn setup() -> SqliteBookmarkRepository {
    let pool = db::init("sqlite::memory:").await.expect("in-memory db init failed");
    SqliteBookmarkRepository::new(pool)
  }

  fn sample_bookmark(id: &str) -> Bookmark {
    Bookmark {
      id: id.to_string(),
      url: "https://example.com/".to_string(),
      url_normalized: "https://example.com/".to_string(),
      title: "Test".to_string(),
      description: String::new(),
      tags: vec!["rust".to_string()],
      notes: String::new(),
      status: BookmarkStatus::Unread,
      category: BookmarkCategory::Queue,
      character_count: None,
      browser_bookmark_id: None,
      deleted_at: None,
      created_at: "2026-01-01T00:00:00.000Z".to_string(),
      updated_at: "2026-01-01T00:00:00.000Z".to_string(),
    }
  }

  #[tokio::test]
  async fn create_inserts_row() {
    let repo = setup().await;
    repo.create(&sample_bookmark("id-1")).await.unwrap();
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bookmarks WHERE id = ?")
      .bind("id-1")
      .fetch_one(&repo.pool)
      .await
      .unwrap();
    assert_eq!(count, 1);
  }

  #[tokio::test]
  async fn get_by_id_returns_bookmark() {
    let repo = setup().await;
    repo.create(&sample_bookmark("id-1")).await.unwrap();
    let found = repo.get_by_id("id-1").await.unwrap().unwrap();
    assert_eq!(found.id, "id-1");
    assert_eq!(found.tags, vec!["rust"]);
  }

  #[tokio::test]
  async fn get_by_id_missing_returns_none() {
    let repo = setup().await;
    assert!(repo.get_by_id("no-such-id").await.unwrap().is_none());
  }

  #[tokio::test]
  async fn create_duplicate_url_normalized_returns_duplicate() {
    let repo = setup().await;
    let b1 = sample_bookmark("id-1");
    let mut b2 = sample_bookmark("id-2");
    b2.url = "https://example.com/?utm_source=x".to_string(); // different raw URL
    repo.create(&b1).await.unwrap();
    let err = repo.create(&b2).await.unwrap_err();
    assert!(matches!(err, AppError::Duplicate(_)));
  }
}
