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

        let mut stmt = db.prepare("PRAGMA table_info(snippets)")?;
        let columns: Vec<String> = stmt
            .query_map([], |row| row.get(1))?
            .collect::<Result<Vec<_>, _>>()?;

        if !columns.contains(&"times_used".to_string()) {
            db.execute(
                "ALTER TABLE snippets ADD COLUMN times_used INTEGER NOT NULL DEFAULT 0",
                [],
            )?;
        }
        if !columns.contains(&"last_used_at".to_string()) {
            db.execute(
                "ALTER TABLE snippets ADD COLUMN last_used_at INTEGER NOT NULL DEFAULT 0",
                [],
            )?;
        }

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
            "INSERT INTO snippets (name, keyword, content, created_at, updated_at, times_used, last_used_at)
             VALUES (?1, ?2, ?3, ?4, ?4, 0, 0)",
            params![name, keyword, content, now],
        )?;
        Ok(db.last_insert_rowid())
    }

    pub fn list_snippets(&self, search_term: Option<String>) -> Result<Vec<Snippet>, AppError> {
        let db = self.db.lock().unwrap();
        let mut query = "SELECT id, name, keyword, content, created_at, updated_at, times_used, last_used_at FROM snippets".to_string();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        if let Some(term) = search_term {
            if !term.is_empty() {
                query.push_str(" WHERE name LIKE ?1 OR keyword LIKE ?1 OR content LIKE ?1");
                params_vec.push(Box::new(format!("%{}%", term)));
            }
        }

        query.push_str(" ORDER BY updated_at DESC");

        let params_ref: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

        let mut stmt = db.prepare(&query)?;
        let snippets_iter = stmt.query_map(&params_ref[..], |row| {
            let created_at_ts: i64 = row.get(4)?;
            let updated_at_ts: i64 = row.get(5)?;
            let last_used_at_ts: i64 = row.get(7)?;
            Ok(Snippet {
                id: row.get(0)?,
                name: row.get(1)?,
                keyword: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or_default(),
                updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or_default(),
                times_used: row.get(6)?,
                last_used_at: DateTime::from_timestamp(last_used_at_ts, 0).unwrap_or_default(),
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

    pub fn snippet_was_used(&self, id: i64) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        let now = Utc::now().timestamp();
        db.execute(
            "UPDATE snippets SET times_used = times_used + 1, last_used_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn find_snippet_by_keyword(&self, keyword: &str) -> Result<Option<Snippet>, AppError> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare("SELECT id, name, keyword, content, created_at, updated_at, times_used, last_used_at FROM snippets WHERE keyword = ?1")?;
        let mut rows = stmt.query_map(params![keyword], |row| {
            let created_at_ts: i64 = row.get(4)?;
            let updated_at_ts: i64 = row.get(5)?;
            let last_used_at_ts: i64 = row.get(7)?;
            Ok(Snippet {
                id: row.get(0)?,
                name: row.get(1)?,
                keyword: row.get(2)?,
                content: row.get(3)?,
                created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or_default(),
                updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or_default(),
                times_used: row.get(6)?,
                last_used_at: DateTime::from_timestamp(last_used_at_ts, 0).unwrap_or_default(),
            })
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }
}