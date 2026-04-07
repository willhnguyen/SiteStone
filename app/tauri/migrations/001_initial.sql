-- bookmarks table
CREATE TABLE IF NOT EXISTS bookmarks (
  id TEXT PRIMARY KEY NOT NULL,
  url TEXT NOT NULL,
  url_normalized TEXT NOT NULL,
  title TEXT NOT NULL DEFAULT '',
  description TEXT NOT NULL DEFAULT '',
  tags TEXT NOT NULL DEFAULT '[]',
  notes TEXT NOT NULL DEFAULT '',
  status TEXT NOT NULL DEFAULT 'unread'
    CHECK (status IN ('unread', 'read', 'skipped')),
  category TEXT NOT NULL DEFAULT 'queue'
    CHECK (category IN ('queue', 'reference', 'youtube', 'trash')),
  character_count INTEGER,
  browser_bookmark_id TEXT,
  deleted_at TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_bookmarks_url_normalized
  ON bookmarks (url_normalized)
  WHERE deleted_at IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_bookmarks_browser_id
  ON bookmarks (browser_bookmark_id)
  WHERE browser_bookmark_id IS NOT NULL AND deleted_at IS NULL;

-- sync_ops table
CREATE TABLE IF NOT EXISTS sync_ops (
  id TEXT PRIMARY KEY NOT NULL,
  op_type TEXT NOT NULL CHECK (op_type IN ('create', 'update', 'delete')),
  bookmark_id TEXT NOT NULL,
  field TEXT,
  old_value TEXT,
  new_value TEXT,
  wpm REAL,
  time_spent INTEGER,
  timestamp TEXT NOT NULL,
  source TEXT NOT NULL CHECK (source IN ('browser', 'app')),
  seq INTEGER NOT NULL,
  applied INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_sync_ops_source_seq
  ON sync_ops (source, seq);

CREATE INDEX IF NOT EXISTS idx_sync_ops_bookmark_id
  ON sync_ops (bookmark_id);

CREATE INDEX IF NOT EXISTS idx_sync_ops_timestamp
  ON sync_ops (timestamp);

-- user_metrics table
CREATE TABLE IF NOT EXISTS user_metrics (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  wpm_samples TEXT NOT NULL DEFAULT '[]',
  avg_wpm REAL NOT NULL DEFAULT 0.0,
  last_updated TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

INSERT OR IGNORE INTO user_metrics (id) VALUES (1);

-- settings table
CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY NOT NULL,
  value TEXT NOT NULL,
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

INSERT OR IGNORE INTO settings (key, value) VALUES
  ('soft_delete_ttl_days', '30'),
  ('target_per_day', '20'),
  ('sync_poll_interval_seconds', '60'),
  ('embedding_model', '"all-MiniLM-L6-v2"'),
  ('embedding_dim', '384');

-- daily_metrics table
CREATE TABLE IF NOT EXISTS daily_metrics (
  date TEXT PRIMARY KEY NOT NULL,
  bookmarks_read INTEGER NOT NULL DEFAULT 0,
  bookmarks_skipped INTEGER NOT NULL DEFAULT 0,
  bookmarks_trashed INTEGER NOT NULL DEFAULT 0,
  avg_wpm REAL,
  total_reading_sec INTEGER NOT NULL DEFAULT 0
);

-- bookmark_embeddings virtual table created in code after vec0 extension is loaded (see sqlite/db.rs)
