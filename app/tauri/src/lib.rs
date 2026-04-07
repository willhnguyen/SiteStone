mod commands;
mod domain;
mod dto;
mod error;
mod repository;
mod service;
mod sqlite;

use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tauri::Manager;

#[derive(Clone)]
pub struct AppState {
  db: Arc<SqlitePool>,
}

impl AppState {
  pub fn new(db: SqlitePool) -> Self {
    Self { db: Arc::new(db) }
  }

  pub fn db(&self) -> &SqlitePool {
    &self.db
  }
}

pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let home_dir = dirs::home_dir().ok_or("failed to get home directory")?;
      let app_dir = home_dir.join(".sitestone");

      std::fs::create_dir_all(&app_dir).ok();

      let db_url = format!("sqlite:{}", app_dir.join("sitestone.db").display());

      let runtime = tokio::runtime::Runtime::new()
        .map_err(|_| "failed to create tokio runtime")?;
      let db = runtime.block_on(async {
        sqlite::db::init(&db_url)
          .await
          .map_err(|e| format!("failed to initialize database: {}", e))
      })?;

      let state = AppState::new(db);
      app.manage(state);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::bookmarks::add_bookmark,
      commands::bookmarks::get_bookmark,
      commands::bookmarks::get_bookmarks,
      commands::bookmarks::update_bookmark,
      commands::bookmarks::delete_bookmark,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
