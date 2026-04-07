use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("database error: {0}")]
  Database(#[from] sqlx::Error),

  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  #[error("serialization error: {0}")]
  Serialization(#[from] serde_json::Error),

  #[error("not found")]
  NotFound,

  #[error("duplicate: {0}")]
  Duplicate(String),

  #[error("invalid input: {0}")]
  InvalidInput(String),
}

impl serde::ser::Serialize for AppError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}
