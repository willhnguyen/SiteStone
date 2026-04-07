use crate::error::AppError;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

pub async fn init(db_url: &str) -> Result<SqlitePool, AppError> {
  let connect_options = SqliteConnectOptions::from_str(db_url)?
    .create_if_missing(true)
    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

  let pool = SqlitePoolOptions::new()
    .max_connections(5)
    .connect_with(connect_options)
    .await?;

  // Run migrations
  sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .map_err(|e| AppError::Database(sqlx::Error::Io(std::io::Error::other(e.to_string()))))?;

  // Load vec0 extension and scaffold bookmark_embeddings virtual table.
  // Both steps are silently skipped if the extension is unavailable (test environments).
  if sqlx::query("SELECT load_extension('vec0')")
    .execute(&pool)
    .await
    .is_ok()
  {
    sqlx::query(
      "CREATE VIRTUAL TABLE IF NOT EXISTS bookmark_embeddings USING vec0(
        bookmark_id TEXT PRIMARY KEY,
        embedding FLOAT[384]
      )",
    )
    .execute(&pool)
    .await
    .ok();
  }

  Ok(pool)
}
