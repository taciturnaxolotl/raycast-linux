use super::{
    encryption::{decrypt, encrypt, get_encryption_key},
    monitor::start_monitoring,
    types::{ClipboardItem, ContentType, INLINE_CONTENT_THRESHOLD_BYTES, PREVIEW_LENGTH_CHARS},
};
use crate::error::AppError;
use chrono::Utc;
use once_cell::sync::Lazy;
use rusqlite::{params, Connection, Result as RusqliteResult};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct ClipboardHistoryManager {
    db: Mutex<Connection>,
    key: [u8; 32],
    pub image_dir: PathBuf,
}

impl ClipboardHistoryManager {
    fn new(app_handle: AppHandle) -> Result<Self, AppError> {
        let data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|_| AppError::DirectoryNotFound)?;
        let image_dir = data_dir.join("clipboard_images");
        std::fs::create_dir_all(&image_dir)?;

        let db_path = data_dir.join("clipboard_history.sqlite");
        let db = Connection::open(db_path)?;

        let key = get_encryption_key()?;

        Ok(Self {
            db: Mutex::new(db),
            key,
            image_dir,
        })
    }

    fn init_db(&self) -> RusqliteResult<()> {
        let db = self.db.lock().unwrap();
        db.execute(
            "CREATE TABLE IF NOT EXISTS clipboard_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                hash TEXT UNIQUE NOT NULL,
                content_type TEXT NOT NULL,
                encrypted_content TEXT NOT NULL,
                encrypted_preview TEXT,
                content_size_bytes INTEGER,
                source_app_name TEXT,
                first_copied_at INTEGER NOT NULL,
                last_copied_at INTEGER NOT NULL,
                times_copied INTEGER NOT NULL DEFAULT 1,
                is_pinned INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_item(
        &self,
        hash: String,
        content_type: ContentType,
        content_value: String,
        source_app_name: Option<String>,
    ) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        let now = Utc::now();

        let existing_item: RusqliteResult<i64> = db.query_row(
            "SELECT id FROM clipboard_history WHERE hash = ?",
            params![&hash],
            |row| row.get(0),
        );

        if let Ok(_id) = existing_item {
            db.execute(
                "UPDATE clipboard_history SET last_copied_at = ?, times_copied = times_copied + 1 WHERE hash = ?",
                params![now.timestamp(), &hash],
            )?;
        } else {
            let content_size_bytes = content_value.len() as i64;
            let mut preview_text = content_value
                .chars()
                .take(PREVIEW_LENGTH_CHARS)
                .collect::<String>();
            if content_value.chars().count() > PREVIEW_LENGTH_CHARS {
                preview_text.push_str("...");
            }

            let encrypted_preview = encrypt(&preview_text, &self.key)?;
            let encrypted_content = encrypt(&content_value, &self.key)?;
            db.execute(
                "INSERT INTO clipboard_history (hash, content_type, encrypted_content, encrypted_preview, content_size_bytes, source_app_name, first_copied_at, last_copied_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                params![hash, content_type.as_str(), encrypted_content, encrypted_preview, content_size_bytes, source_app_name, now.timestamp(), now.timestamp()],
            )?;
        }
        Ok(())
    }

    pub fn get_items(
        &self,
        filter: String,
        search_term: Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<ClipboardItem>, AppError> {
        let db = self.db.lock().unwrap();
        let mut query = "SELECT id, hash, content_type, source_app_name, first_copied_at, last_copied_at, times_copied, is_pinned, content_size_bytes, encrypted_preview, CASE WHEN content_size_bytes <= ? THEN encrypted_content ELSE NULL END as conditional_encrypted_content FROM clipboard_history".to_string();
        let mut where_clauses: Vec<String> = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> =
            vec![Box::new(INLINE_CONTENT_THRESHOLD_BYTES)];

        match filter.as_str() {
            "pinned" => where_clauses.push("is_pinned = 1".to_string()),
            "text" => where_clauses.push("content_type = 'text'".to_string()),
            "image" => where_clauses.push("content_type = 'image'".to_string()),
            "link" => where_clauses.push("content_type = 'link'".to_string()),
            "color" => where_clauses.push("content_type = 'color'".to_string()),
            _ => {}
        }

        if !where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&where_clauses.join(" AND "));
        }

        query.push_str(" ORDER BY last_copied_at DESC LIMIT ? OFFSET ?");
        params_vec.push(Box::new(limit));
        params_vec.push(Box::new(offset));

        let params_ref: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();

        let mut stmt = db.prepare(&query)?;
        let items_iter = stmt.query_map(&params_ref[..], |row| {
            let conditional_encrypted_content: Option<String> = row.get(10)?;
            let content_value =
                conditional_encrypted_content.and_then(|cec| decrypt(&cec, &self.key).ok());

            let encrypted_preview: Option<String> = row.get(9)?;
            let preview = encrypted_preview.and_then(|ep| decrypt(&ep, &self.key).ok());

            let first_ts: i64 = row.get(4)?;
            let last_ts: i64 = row.get(5)?;

            Ok(ClipboardItem {
                id: row.get(0)?,
                hash: row.get(1)?,
                content_type: ContentType::from_str(&row.get::<_, String>(2)?)
                    .unwrap_or(ContentType::Text),
                content_value,
                preview,
                content_size_bytes: row.get(8)?,
                source_app_name: row.get(3)?,
                first_copied_at: chrono::DateTime::from_timestamp(first_ts, 0).unwrap_or_default(),
                last_copied_at: chrono::DateTime::from_timestamp(last_ts, 0).unwrap_or_default(),
                times_copied: row.get(6)?,
                is_pinned: row.get::<_, i32>(7)? == 1,
            })
        })?;

        let mut all_items = items_iter.collect::<Result<Vec<_>, _>>()?;

        if let Some(term) = search_term {
            if !term.is_empty() {
                let lower_term = term.to_lowercase();
                all_items.retain(|item| {
                    if let Some(preview) = &item.preview {
                        preview.to_lowercase().contains(&lower_term)
                    } else if let Some(value) = &item.content_value {
                        value.to_lowercase().contains(&lower_term)
                    } else {
                        false
                    }
                });
            }
        }

        Ok(all_items)
    }

    pub fn get_item_content(&self, id: i64) -> Result<String, AppError> {
        let db = self.db.lock().unwrap();
        let encrypted_content: String = db.query_row(
            "SELECT encrypted_content FROM clipboard_history WHERE id = ?",
            params![id],
            |row| row.get(0),
        )?;
        decrypt(&encrypted_content, &self.key)
    }

    pub fn item_was_copied(&self, id: i64) -> RusqliteResult<usize> {
        self.db.lock().unwrap().execute(
            "UPDATE clipboard_history SET last_copied_at = ?, times_copied = times_copied + 1 WHERE id = ?",
            params![Utc::now().timestamp(), id],
        )
    }

    pub fn delete_item(&self, id: i64) -> RusqliteResult<usize> {
        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM clipboard_history WHERE id = ?", params![id])
    }

    pub fn toggle_pin(&self, id: i64) -> RusqliteResult<usize> {
        self.db.lock().unwrap().execute(
            "UPDATE clipboard_history SET is_pinned = 1 - is_pinned WHERE id = ?",
            params![id],
        )
    }

    pub fn clear_all(&self) -> RusqliteResult<usize> {
        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM clipboard_history WHERE is_pinned = 0", [])
    }
}

pub static MANAGER: Lazy<Mutex<Option<ClipboardHistoryManager>>> = Lazy::new(|| Mutex::new(None));
pub static INTERNAL_CLIPBOARD_CHANGE: AtomicBool = AtomicBool::new(false);

pub fn init(app_handle: AppHandle) {
    let mut manager_guard = MANAGER.lock().unwrap();
    if manager_guard.is_none() {
        match ClipboardHistoryManager::new(app_handle.clone()) {
            Ok(manager) => {
                if let Err(e) = manager.init_db() {
                    eprintln!("Failed to initialize clipboard history database: {:?}", e);
                    return;
                }
                *manager_guard = Some(manager);
                drop(manager_guard);
                start_monitoring(app_handle);
            }
            Err(e) => eprintln!("Failed to create ClipboardHistoryManager: {:?}", e),
        }
    }
}