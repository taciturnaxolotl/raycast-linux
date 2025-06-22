use crate::error::AppError;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as RusqliteResult};
use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_opener::{open_path, open_url};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quicklink {
    id: i64,
    name: String,
    link: String,
    application: Option<String>,
    icon: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct QuicklinkManager {
    db: Mutex<Connection>,
}

impl QuicklinkManager {
    pub fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|_| AppError::DirectoryNotFound)?;
        let db_path = data_dir.join("quicklinks.sqlite");
        let db = Connection::open(db_path)?;

        Ok(Self {
            db: Mutex::new(db),
        })
    }

    pub fn init_db(&self) -> RusqliteResult<()> {
        let db = self.db.lock().unwrap();
        db.execute(
            "CREATE TABLE IF NOT EXISTS quicklinks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                link TEXT NOT NULL,
                application TEXT,
                icon TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    fn create_quicklink(
        &self,
        name: String,
        link: String,
        application: Option<String>,
        icon: Option<String>,
    ) -> Result<i64, AppError> {
        let db = self.db.lock().unwrap();
        let now = Utc::now().timestamp();
        db.execute(
            "INSERT INTO quicklinks (name, link, application, icon, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![name, link, application, icon, now, now],
        )?;
        Ok(db.last_insert_rowid())
    }

    fn list_quicklinks(&self) -> Result<Vec<Quicklink>, AppError> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare("SELECT id, name, link, application, icon, created_at, updated_at FROM quicklinks ORDER BY name ASC")?;
        let quicklinks_iter = stmt.query_map([], |row| {
            let created_at_ts: i64 = row.get(5)?;
            let updated_at_ts: i64 = row.get(6)?;
            Ok(Quicklink {
                id: row.get(0)?,
                name: row.get(1)?,
                link: row.get(2)?,
                application: row.get(3)?,
                icon: row.get(4)?,
                created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or_default(),
                updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or_default(),
            })
        })?;

        quicklinks_iter.collect::<Result<Vec<_>, _>>().map_err(|e| e.into())
    }

    fn update_quicklink(
        &self,
        id: i64,
        name: String,
        link: String,
        application: Option<String>,
        icon: Option<String>,
    ) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        let now = Utc::now().timestamp();
        db.execute(
            "UPDATE quicklinks SET name = ?, link = ?, application = ?, icon = ?, updated_at = ?
             WHERE id = ?",
            params![name, link, application, icon, now, id],
        )?;
        Ok(())
    }

    fn delete_quicklink(&self, id: i64) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        db.execute("DELETE FROM quicklinks WHERE id = ?", params![id])?;
        Ok(())
    }
}

#[tauri::command]
pub fn create_quicklink(
    app: AppHandle,
    name: String,
    link: String,
    application: Option<String>,
    icon: Option<String>,
) -> Result<i64, String> {
    app.state::<QuicklinkManager>()
        .create_quicklink(name, link, application, icon)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_quicklinks(app: AppHandle) -> Result<Vec<Quicklink>, String> {
    app.state::<QuicklinkManager>()
        .list_quicklinks()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_quicklink(
    app: AppHandle,
    id: i64,
    name: String,
    link: String,
    application: Option<String>,
    icon: Option<String>,
) -> Result<(), String> {
    app.state::<QuicklinkManager>()
        .update_quicklink(id, name, link, application, icon)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_quicklink(app: AppHandle, id: i64) -> Result<(), String> {
    app.state::<QuicklinkManager>()
        .delete_quicklink(id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn execute_quicklink(link: String, application: Option<String>) -> Result<(), String> {
    if let Some(app_name) = application {
        open_path(link, Some(app_name)).map_err(|e| e.to_string())
    } else if link.starts_with("http://") || link.starts_with("https://") {
        open_url(link, None::<String>).map_err(|e| e.to_string())
    } else {
        open_path(link, None::<String>).map_err(|e| e.to_string())
    }
}