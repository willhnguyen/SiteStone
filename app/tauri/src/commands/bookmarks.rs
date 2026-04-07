use crate::dto::bookmark::{BookmarkDto, CreateBookmarkRequest};
use crate::error::AppError;
use crate::service::bookmark::{BookmarkService, CreateBookmarkParams};
use crate::sqlite::bookmark::SqliteBookmarkRepository;
use crate::AppState;
use tauri::State;

fn svc(state: &AppState) -> BookmarkService<SqliteBookmarkRepository> {
  BookmarkService::new(SqliteBookmarkRepository::new(state.db().clone()))
}

#[tauri::command]
pub async fn add_bookmark(
  state: State<'_, AppState>,
  req: CreateBookmarkRequest,
) -> Result<BookmarkDto, AppError> {
  let params = CreateBookmarkParams {
    url: req.url,
    title: req.title.unwrap_or_default(),
    description: req.description.unwrap_or_default(),
    tags: req.tags.unwrap_or_default(),
    notes: req.notes.unwrap_or_default(),
    browser_bookmark_id: req.browser_bookmark_id,
  };
  svc(&state).create(params).await.map(BookmarkDto::from)
}
