use crate::dto::bookmark::{BookmarkDto, CreateBookmarkRequest, ListBookmarksRequest, UpdateBookmarkRequest};
use crate::repository::bookmark::BookmarkFilter;
use crate::error::AppError;
use crate::service::bookmark::{BookmarkService, CreateBookmarkParams, UpdateBookmarkParams};
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

#[tauri::command]
pub async fn restore_bookmark(
  state: State<'_, AppState>,
  id: String,
) -> Result<BookmarkDto, AppError> {
  svc(&state).restore(&id).await.map(BookmarkDto::from)
}

#[tauri::command]
pub async fn delete_bookmark(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
  svc(&state).soft_delete(&id).await
}

#[tauri::command]
pub async fn update_bookmark(
  state: State<'_, AppState>,
  id: String,
  req: UpdateBookmarkRequest,
) -> Result<BookmarkDto, AppError> {
  let params = UpdateBookmarkParams {
    title: req.title,
    description: req.description,
    tags: req.tags,
    notes: req.notes,
    status: req.status,
    category: req.category,
  };
  svc(&state).update(&id, params).await.map(BookmarkDto::from)
}

#[tauri::command]
pub async fn get_bookmarks(
  state: State<'_, AppState>,
  req: ListBookmarksRequest,
) -> Result<Vec<BookmarkDto>, AppError> {
  let filter = BookmarkFilter {
    status: req.status,
    category: req.category,
    include_deleted: req.include_deleted.unwrap_or(false),
    limit: req.limit,
    offset: req.offset,
  };
  svc(&state)
    .list(filter)
    .await
    .map(|v| v.into_iter().map(BookmarkDto::from).collect())
}

#[tauri::command]
pub async fn get_bookmark(
  state: State<'_, AppState>,
  id: String,
) -> Result<BookmarkDto, AppError> {
  svc(&state).get(&id).await.map(BookmarkDto::from)
}
