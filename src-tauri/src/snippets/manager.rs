use crate::error::AppError;
use rusqlite::{Connection, Result as RusqliteResult};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct SnippetManager {
    db: Mutex<Connection>,
}

impl SnippetManager {
    pub fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|_| AppError::DirectoryNotFound)?;
        let db_path = data_dir.join("snippets.sqlite");
        let db = Connection::open(db_path)?;
        Ok(Self { db: Mutex::new(db) })
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
}