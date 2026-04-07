use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookmarkStatus {
  Unread,
  Read,
  Skipped,
}

impl BookmarkStatus {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Unread => "unread",
      Self::Read => "read",
      Self::Skipped => "skipped",
    }
  }
}

impl std::fmt::Display for BookmarkStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl std::str::FromStr for BookmarkStatus {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "unread" => Ok(Self::Unread),
      "read" => Ok(Self::Read),
      "skipped" => Ok(Self::Skipped),
      _ => Err(format!("invalid bookmark status: {s}")),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BookmarkCategory {
  Queue,
  Reference,
  Youtube,
  Trash,
}

impl BookmarkCategory {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Queue => "queue",
      Self::Reference => "reference",
      Self::Youtube => "youtube",
      Self::Trash => "trash",
    }
  }
}

impl std::fmt::Display for BookmarkCategory {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl std::str::FromStr for BookmarkCategory {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "queue" => Ok(Self::Queue),
      "reference" => Ok(Self::Reference),
      "youtube" => Ok(Self::Youtube),
      "trash" => Ok(Self::Trash),
      _ => Err(format!("invalid bookmark category: {s}")),
    }
  }
}

#[cfg(test)]
mod status_tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn roundtrip_all_variants() {
    for (variant, s) in [
      (BookmarkStatus::Unread, "unread"),
      (BookmarkStatus::Read, "read"),
      (BookmarkStatus::Skipped, "skipped"),
    ] {
      assert_eq!(variant.to_string(), s);
      assert_eq!(BookmarkStatus::from_str(s).unwrap(), variant);
    }
  }

  #[test]
  fn from_str_invalid_returns_err() {
    assert!(BookmarkStatus::from_str("unknown").is_err());
  }
}

#[cfg(test)]
mod category_tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn roundtrip_all_variants() {
    for (variant, s) in [
      (BookmarkCategory::Queue, "queue"),
      (BookmarkCategory::Reference, "reference"),
      (BookmarkCategory::Youtube, "youtube"),
      (BookmarkCategory::Trash, "trash"),
    ] {
      assert_eq!(variant.to_string(), s);
      assert_eq!(BookmarkCategory::from_str(s).unwrap(), variant);
    }
  }

  #[test]
  fn from_str_invalid_returns_err() {
    assert!(BookmarkCategory::from_str("invalid").is_err());
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
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
