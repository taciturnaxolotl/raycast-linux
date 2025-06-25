use crate::error::AppError;
use chrono::Utc;
use rusqlite::{params, Connection, Result as RusqliteResult};
use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FrecencyData {
    pub item_id: String,
    pub use_count: i64,
    pub last_used_at: i64,
}

pub struct FrecencyManager {
    db: Mutex<Connection>,
}

impl FrecencyManager {
    pub fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|_| AppError::DirectoryNotFound)?;
        let db_path = data_dir.join("frecency.sqlite");
        let db = Connection::open(db_path)?;
        let manager = Self { db: Mutex::new(db) };
        manager.init_db()?;
        Ok(manager)
    }

    fn init_db(&self) -> RusqliteResult<()> {
        let db = self.db.lock().unwrap();
        db.execute(
            "CREATE TABLE IF NOT EXISTS frecency (
                item_id TEXT PRIMARY KEY,
                use_count INTEGER NOT NULL DEFAULT 0,
                last_used_at INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn record_usage(&self, item_id: String) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        let now = Utc::now().timestamp();
        db.execute(
            "INSERT INTO frecency (item_id, use_count, last_used_at) VALUES (?, 1, ?)
             ON CONFLICT(item_id) DO UPDATE SET
                use_count = use_count + 1,
                last_used_at = excluded.last_used_at",
            params![item_id, now],
        )?;
        Ok(())
    }

    pub fn get_frecency_data(&self) -> Result<Vec<FrecencyData>, AppError> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare("SELECT item_id, use_count, last_used_at FROM frecency")?;
        let data_iter = stmt.query_map([], |row| {
            Ok(FrecencyData {
                item_id: row.get(0)?,
                use_count: row.get(1)?,
                last_used_at: row.get(2)?,
            })
        })?;

        data_iter
            .collect::<RusqliteResult<Vec<_>>>()
            .map_err(|e| e.into())
    }
}