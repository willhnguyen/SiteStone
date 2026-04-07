use crate::domain::bookmark::{Bookmark, BookmarkCategory, BookmarkStatus};
use crate::error::AppError;
use crate::repository::bookmark::BookmarkRepository;
use url::Url;
use uuid::Uuid;

pub struct CreateBookmarkParams {
  pub url: String,
  pub title: String,
  pub description: String,
  pub tags: Vec<String>,
  pub notes: String,
  pub browser_bookmark_id: Option<String>,
}

pub struct BookmarkService<R> {
  repo: R,
}

impl<R: BookmarkRepository> BookmarkService<R> {
  pub fn new(repo: R) -> Self {
    Self { repo }
  }

  pub async fn get(&self, id: &str) -> Result<Bookmark, AppError> {
    self.repo.get_by_id(id).await?.ok_or(AppError::NotFound)
  }

  pub async fn create(&self, params: CreateBookmarkParams) -> Result<Bookmark, AppError> {
    let url_normalized = normalize_url(&params.url)?;
    let now = now();
    let bookmark = Bookmark {
      id: Uuid::now_v7().to_string(),
      url: params.url,
      url_normalized,
      title: params.title,
      description: params.description,
      tags: params.tags,
      notes: params.notes,
      status: BookmarkStatus::Unread,
      category: BookmarkCategory::Queue,
      character_count: None,
      browser_bookmark_id: params.browser_bookmark_id,
      deleted_at: None,
      created_at: now.clone(),
      updated_at: now,
    };
    self.repo.create(&bookmark).await?;
    Ok(bookmark)
  }
}

/// Normalizes a URL for deduplication:
/// - Lowercases scheme and host (url crate handles this)
/// - Removes tracking query params: utm_*, fbclid, gclid, ref
/// - Removes fragment
pub fn normalize_url(raw: &str) -> Result<String, AppError> {
  let mut url = Url::parse(raw)
    .map_err(|_| AppError::InvalidInput(format!("invalid URL: {raw}")))?;

  let kept: Vec<(String, String)> = url
    .query_pairs()
    .filter(|(k, _)| {
      !k.starts_with("utm_") && !matches!(k.as_ref(), "fbclid" | "gclid" | "ref")
    })
    .map(|(k, v)| (k.into_owned(), v.into_owned()))
    .collect();

  if kept.is_empty() {
    url.set_query(None);
  } else {
    url.query_pairs_mut().clear().extend_pairs(&kept);
  }

  url.set_fragment(None);

  Ok(url.to_string())
}

fn now() -> String {
  chrono::Utc::now().format("%Y-%m-%dT%H:%M:%3fZ").to_string()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Mutex;

  #[derive(Default)]
  struct FakeBookmarkRepo {
    store: Mutex<Vec<Bookmark>>,
  }

  impl BookmarkRepository for FakeBookmarkRepo {
    async fn create(&self, b: &Bookmark) -> Result<(), AppError> {
      let mut store = self.store.lock().unwrap();
      if store.iter().any(|e| e.url_normalized == b.url_normalized && e.deleted_at.is_none()) {
        return Err(AppError::Duplicate(b.url_normalized.clone()));
      }
      store.push(b.clone());
      Ok(())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Bookmark>, AppError> {
      Ok(self.store.lock().unwrap().iter().find(|b| b.id == id).cloned())
    }
  }

  fn svc() -> BookmarkService<FakeBookmarkRepo> {
    BookmarkService::new(FakeBookmarkRepo::default())
  }

  fn create_params(url: &str) -> CreateBookmarkParams {
    CreateBookmarkParams {
      url: url.to_string(),
      title: "Test".to_string(),
      description: String::new(),
      tags: vec![],
      notes: String::new(),
      browser_bookmark_id: None,
    }
  }

  #[tokio::test]
  async fn get_returns_bookmark() {
    let svc = svc();
    let b = svc.create(create_params("https://example.com")).await.unwrap();
    let found = svc.get(&b.id).await.unwrap();
    assert_eq!(found.id, b.id);
  }

  #[tokio::test]
  async fn get_missing_returns_not_found() {
    let err = svc().get("no-such-id").await.unwrap_err();
    assert!(matches!(err, AppError::NotFound));
  }

  #[tokio::test]
  async fn create_returns_new_bookmark() {
    let b = svc().create(create_params("https://example.com")).await.unwrap();
    assert_eq!(b.url, "https://example.com");
    assert_eq!(b.status, BookmarkStatus::Unread);
    assert_eq!(b.category, BookmarkCategory::Queue);
  }

  #[tokio::test]
  async fn create_duplicate_url_returns_error() {
    let svc = svc();
    svc.create(create_params("https://example.com?utm_source=x")).await.unwrap();
    let err = svc.create(create_params("https://example.com")).await.unwrap_err();
    assert!(matches!(err, AppError::Duplicate(_)));
  }

  #[tokio::test]
  async fn create_invalid_url_returns_error() {
    let err = svc().create(create_params("not-a-url")).await.unwrap_err();
    assert!(matches!(err, AppError::InvalidInput(_)));
  }

  // URL normalization tests

  #[test]
  fn normalize_removes_utm_params() {
    let r = normalize_url("https://example.com?utm_source=x&utm_medium=y&keep=1").unwrap();
    assert!(r.contains("keep=1"));
    assert!(!r.contains("utm_"));
  }

  #[test]
  fn normalize_removes_fbclid_gclid_ref() {
    let r = normalize_url("https://example.com?fbclid=abc&gclid=def&ref=home").unwrap();
    assert!(!r.contains("fbclid"));
    assert!(!r.contains("gclid"));
    assert!(!r.contains("ref="));
  }

  #[test]
  fn normalize_removes_fragment() {
    assert!(!normalize_url("https://example.com/page#section").unwrap().contains('#'));
  }

  #[test]
  fn normalize_lowercases_host() {
    assert!(normalize_url("https://EXAMPLE.COM/path").unwrap().starts_with("https://example.com/"));
  }

  #[test]
  fn normalize_invalid_url_returns_error() {
    assert!(matches!(normalize_url("not-a-url"), Err(AppError::InvalidInput(_))));
  }
}
