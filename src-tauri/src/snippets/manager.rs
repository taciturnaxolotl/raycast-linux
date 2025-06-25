use crate::error::AppError;
use crate::snippets::types::Snippet;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as RusqliteResult};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

#[derive(Clone)]
pub struct SnippetManager {
    db: Arc<Mutex<Connection>>,
}

impl SnippetManager {
    pub fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|_| AppError::DirectoryNotFound)?;
        let db_path = data_dir.join("snippets.sqlite");
        let db = Connection::open(db_path)?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    pub fn init_db(&self) -> RusqliteResult<()> {
        let db = self.db.lock().unwrap();
        db.execute(
            "CREATE TABLE IF NOT EXISTS snippets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                keyword TEXT NOT NULL UNIQUE,
                content TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn create_snippet(
        &self,
        name: String,
        keyword: String,
        content: String,
    ) -> Result<i64, AppError> {
        let db = self.db.lock().unwrap();
        let now = Utc::now().timestamp();
        db.execute(
            "INSERT INTO snippets (name, keyword, content, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?4)",
            params![name, keyword, content, now],
        )?;
        Ok(db.last_insert_rowid())
    }

    pub fn list_snippets(&self) -> Result<Vec<Snippet>, AppError> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare("SELECT id, name, keyword, content, created_at, updated_at FROM snippets ORDER BY name ASC")?;
        let snippets_iter = stmt.query_map([], |row| {
            let created_at_ts: i64 = row.get(4)?;
            let updated_at_ts: i64 = row.get(5)?;
            Ok(Snippet {
                id: row.get(0)?,
                name: row.get(1)?,
                keyword: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or_default(),
                updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or_default(),
            })
        })?;

        snippets_iter
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.into())
    }

    pub fn update_snippet(
        &self,
        id: i64,
        name: String,
        keyword: String,
        content: String,
    ) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        let now = Utc::now().timestamp();
        db.execute(
            "UPDATE snippets SET name = ?1, keyword = ?2, content = ?3, updated_at = ?4 WHERE id = ?5",
            params![name, keyword, content, now, id],
        )?;
        Ok(())
    }

    pub fn delete_snippet(&self, id: i64) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        db.execute("DELETE FROM snippets WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn find_snippet_by_keyword(&self, keyword: &str) -> Result<Option<Snippet>, AppError> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare("SELECT id, name, keyword, content, created_at, updated_at FROM snippets WHERE keyword = ?1")?;
        let mut rows = stmt.query_map(params![keyword], |row| {
            let created_at_ts: i64 = row.get(4)?;
            let updated_at_ts: i64 = row.get(5)?;
            Ok(Snippet {
                id: row.get(0)?,
                name: row.get(1)?,
                keyword: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or_default(),
                updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or_default(),
            })
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }
}
