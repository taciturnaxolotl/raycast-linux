use crate::error::AppError;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use chrono::{DateTime, NaiveDateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::{params, Connection, Result as RusqliteResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Manager};

const KEYRING_SERVICE: &str = "dev.byteatatime.raycast";
const KEYRING_USERNAME: &str = "clipboard_history_key";

static COLOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap());
static URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap());

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardItem {
    id: i64,
    hash: String,
    content_type: ContentType,
    content_value: String,
    source_app_name: Option<String>,
    first_copied_at: DateTime<Utc>,
    last_copied_at: DateTime<Utc>,
    times_copied: i32,
    is_pinned: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ContentType {
    Text,
    Image,
    Color,
    Link,
    File,
}

impl ContentType {
    fn from_str(s: &str) -> Result<Self, AppError> {
        match s {
            "text" => Ok(ContentType::Text),
            "image" => Ok(ContentType::Image),
            "color" => Ok(ContentType::Color),
            "link" => Ok(ContentType::Link),
            "file" => Ok(ContentType::File),
            _ => Err(AppError::ClipboardHistory("Invalid content type".into())),
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            ContentType::Text => "text",
            ContentType::Image => "image",
            ContentType::Color => "color",
            ContentType::Link => "link",
            ContentType::File => "file",
        }
    }
}

fn get_encryption_key() -> Result<[u8; 32], AppError> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)?;
    match entry.get_password() {
        Ok(hex_key) => {
            let key_bytes =
                hex::decode(hex_key).map_err(|e| AppError::ClipboardHistory(e.to_string()))?;
            Ok(key_bytes.try_into().unwrap())
        }
        Err(keyring::Error::NoEntry) => {
            let new_key: [u8; 32] = rand::random();
            let hex_key = hex::encode(new_key);
            entry.set_password(&hex_key)?;
            Ok(new_key)
        }
        Err(e) => Err(e.into()),
    }
}

fn encrypt(data: &str, key: &[u8; 32]) -> Result<String, AppError> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| AppError::ClipboardHistory(e.to_string()))?;

    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(hex::encode(result))
}

fn decrypt(hex_data: &str, key: &[u8; 32]) -> Result<String, AppError> {
    let data = hex::decode(hex_data).map_err(|e| AppError::ClipboardHistory(e.to_string()))?;
    if data.len() < 12 {
        return Err(AppError::ClipboardHistory("Invalid encrypted data".into()));
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(key.into());
    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| AppError::ClipboardHistory(e.to_string()))?;
    String::from_utf8(decrypted_bytes).map_err(|e| AppError::ClipboardHistory(e.to_string()))
}

pub struct ClipboardHistoryManager {
    db: Mutex<Connection>,
    key: [u8; 32],
    image_dir: PathBuf,
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

    fn add_item(
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
            let encrypted_content = encrypt(&content_value, &self.key)?;
            db.execute(
                "INSERT INTO clipboard_history (hash, content_type, encrypted_content, source_app_name, first_copied_at, last_copied_at)
                 VALUES (?, ?, ?, ?, ?, ?)",
                params![hash, content_type.as_str(), encrypted_content, source_app_name, now.timestamp(), now.timestamp()],
            )?;
        }
        Ok(())
    }

    fn get_items(&self, filter: String, limit: u32) -> Result<Vec<ClipboardItem>, AppError> {
        let db = self.db.lock().unwrap();
        let mut query = "SELECT id, hash, content_type, encrypted_content, source_app_name, first_copied_at, last_copied_at, times_copied, is_pinned FROM clipboard_history".to_string();

        match filter.as_str() {
            "all" => {}
            "pinned" => query.push_str(" WHERE is_pinned = 1"),
            "text" => query.push_str(" WHERE content_type = 'text'"),
            "image" => query.push_str(" WHERE content_type = 'image'"),
            "link" => query.push_str(" WHERE content_type = 'link'"),
            "color" => query.push_str(" WHERE content_type = 'color'"),
            _ => {}
        }

        query.push_str(" ORDER BY last_copied_at DESC LIMIT ?");

        let mut stmt = db.prepare(&query)?;
        let items = stmt.query_map(params![limit], |row| {
            let encrypted_content: String = row.get(3)?;
            let first_ts: i64 = row.get(5)?;
            let last_ts: i64 = row.get(6)?;

            Ok(ClipboardItem {
                id: row.get(0)?,
                hash: row.get(1)?,
                content_type: ContentType::from_str(&row.get::<_, String>(2)?)
                    .unwrap_or(ContentType::Text),
                content_value: decrypt(&encrypted_content, &self.key).unwrap_or_default(),
                source_app_name: row.get(4)?,
                first_copied_at: DateTime::from_naive_utc_and_offset(
                    NaiveDateTime::from_timestamp_opt(first_ts, 0).unwrap_or_default(),
                    Utc,
                ),
                last_copied_at: DateTime::from_naive_utc_and_offset(
                    NaiveDateTime::from_timestamp_opt(last_ts, 0).unwrap_or_default(),
                    Utc,
                ),
                times_copied: row.get(7)?,
                is_pinned: row.get::<_, i32>(8)? == 1,
            })
        })?;

        items.collect::<Result<Vec<_>, _>>().map_err(|e| e.into())
    }

    fn delete_item(&self, id: i64) -> RusqliteResult<usize> {
        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM clipboard_history WHERE id = ?", params![id])
    }

    fn toggle_pin(&self, id: i64) -> RusqliteResult<usize> {
        self.db.lock().unwrap().execute(
            "UPDATE clipboard_history SET is_pinned = 1 - is_pinned WHERE id = ?",
            params![id],
        )
    }

    fn clear_all(&self) -> RusqliteResult<usize> {
        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM clipboard_history WHERE is_pinned = 0", [])
    }
}

static MANAGER: Lazy<Mutex<Option<ClipboardHistoryManager>>> = Lazy::new(|| Mutex::new(None));

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

fn start_monitoring(_app_handle: AppHandle) {
    std::thread::spawn(move || {
        let mut last_text_hash = String::new();
        let mut last_image_hash = String::new();
        let mut clipboard = arboard::Clipboard::new().unwrap();

        loop {
            if let Ok(text) = clipboard.get_text() {
                let text = text.trim();
                if !text.is_empty() {
                    let current_hash = hex::encode(Sha256::digest(text.as_bytes()));
                    if current_hash != last_text_hash {
                        let (content_type, content_value) = if COLOR_REGEX.is_match(text) {
                            (ContentType::Color, text.to_string())
                        } else if URL_REGEX.is_match(text) {
                            (ContentType::Link, text.to_string())
                        } else {
                            (ContentType::Text, text.to_string())
                        };

                        if let Some(manager) = MANAGER.lock().unwrap().as_ref() {
                            if let Err(e) = manager.add_item(
                                current_hash.clone(),
                                content_type,
                                content_value,
                                None,
                            ) {
                                eprintln!("Error adding clipboard text item: {:?}", e);
                            }
                        }
                        last_text_hash = current_hash;
                        last_image_hash.clear();
                    }
                }
            }

            if let Ok(image_data) = clipboard.get_image() {
                let current_hash = hex::encode(Sha256::digest(&image_data.bytes));
                if current_hash != last_image_hash {
                    if let Some(manager) = MANAGER.lock().unwrap().as_ref() {
                        let image_path = manager.image_dir.join(format!("{}.png", &current_hash));
                        match image::save_buffer(
                            &image_path,
                            &image_data.bytes,
                            image_data.width as u32,
                            image_data.height as u32,
                            image::ColorType::Rgba8,
                        ) {
                            Ok(_) => {
                                let content_value = image_path.to_string_lossy().to_string();
                                if let Err(e) = manager.add_item(
                                    current_hash.clone(),
                                    ContentType::Image,
                                    content_value,
                                    None,
                                ) {
                                    eprintln!("Error adding clipboard image item: {:?}", e);
                                }
                            }
                            Err(e) => eprintln!("Failed to save image: {:?}", e),
                        }
                    }
                    last_image_hash = current_hash;
                    last_text_hash.clear();
                }
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    });
}

#[tauri::command]
pub fn history_get_items(filter: String, limit: u32) -> Result<Vec<ClipboardItem>, String> {
    if let Some(manager) = MANAGER.lock().unwrap().as_ref() {
        manager.get_items(filter, limit).map_err(|e| e.to_string())
    } else {
        Err("Clipboard history manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn history_delete_item(id: i64) -> Result<(), String> {
    if let Some(manager) = MANAGER.lock().unwrap().as_ref() {
        manager.delete_item(id).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Clipboard history manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn history_toggle_pin(id: i64) -> Result<(), String> {
    if let Some(manager) = MANAGER.lock().unwrap().as_ref() {
        manager.toggle_pin(id).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Clipboard history manager not initialized".to_string())
    }
}

#[tauri::command]
pub fn history_clear_all() -> Result<(), String> {
    if let Some(manager) = MANAGER.lock().unwrap().as_ref() {
        manager.clear_all().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Clipboard history manager not initialized".to_string())
    }
}