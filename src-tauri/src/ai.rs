use crate::error::AppError;
use futures_util::StreamExt;
use rusqlite::{params, Connection, Result as RusqliteResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

const AI_KEYRING_SERVICE: &str = "dev.byteatatime.raycast.ai";
const AI_KEYRING_USERNAME: &str = "openrouter_api_key";

// --- Structs for API and Events ---

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AskOptions {
    pub model: Option<String>,
    pub creativity: Option<String>,
    #[serde(default)]
    model_mappings: HashMap<String, String>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamChunk {
    request_id: String,
    text: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StreamEnd {
    request_id: String,
    full_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerationData {
    pub id: String,
    pub created: i64,
    pub model: String,
    #[serde(default)]
    pub tokens_prompt: i64,
    #[serde(default)]
    pub tokens_completion: i64,
    #[serde(default)]
    pub native_tokens_prompt: i64,
    #[serde(default)]
    pub native_tokens_completion: i64,
    #[serde(default)]
    pub total_cost: f64,
}

// --- Key Management Commands ---

fn get_keyring_entry() -> Result<keyring::Entry, AppError> {
    keyring::Entry::new(AI_KEYRING_SERVICE, AI_KEYRING_USERNAME).map_err(AppError::from)
}

#[tauri::command]
pub fn set_ai_api_key(key: String) -> Result<(), String> {
    get_keyring_entry()
        .and_then(|entry| entry.set_password(&key).map_err(AppError::from))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_ai_api_key_set() -> Result<bool, String> {
    match get_keyring_entry().and_then(|entry| entry.get_password().map_err(AppError::from)) {
        Ok(_) => Ok(true),
        Err(AppError::Keyring(keyring::Error::NoEntry)) => Ok(false),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn clear_ai_api_key() -> Result<(), String> {
    get_keyring_entry()
        .and_then(|entry| entry.delete_credential().map_err(AppError::from))
        .map_err(|e| e.to_string())
}

// --- Usage Tracking ---

pub struct AiUsageManager {
    db: Mutex<Connection>,
}

impl AiUsageManager {
    pub fn new(app_handle: &AppHandle) -> Result<Self, AppError> {
        let data_dir = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|_| AppError::DirectoryNotFound)?;
        let db_path = data_dir.join("ai_usage.sqlite");
        let db = Connection::open(db_path)?;
        Ok(Self { db: Mutex::new(db) })
    }

    pub fn init_db(&self) -> RusqliteResult<()> {
        let db = self.db.lock().unwrap();
        db.execute(
            "CREATE TABLE IF NOT EXISTS ai_generations (
                id TEXT PRIMARY KEY,
                created INTEGER NOT NULL,
                model TEXT NOT NULL,
                tokens_prompt INTEGER NOT NULL,
                tokens_completion INTEGER NOT NULL,
                native_tokens_prompt INTEGER NOT NULL,
                native_tokens_completion INTEGER NOT NULL,
                total_cost REAL NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn log_generation(&self, data: &GenerationData) -> Result<(), AppError> {
        let db = self.db.lock().unwrap();
        db.execute(
            "INSERT OR REPLACE INTO ai_generations (id, created, model, tokens_prompt, tokens_completion, native_tokens_prompt, native_tokens_completion, total_cost)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                data.id,
                data.created,
                data.model,
                data.tokens_prompt,
                data.tokens_completion,
                data.native_tokens_prompt,
                data.native_tokens_completion,
                data.total_cost
            ],
        )?;
        Ok(())
    }

    pub fn get_history(&self, limit: u32, offset: u32) -> Result<Vec<GenerationData>, AppError> {
        let db = self.db.lock().unwrap();
        let mut stmt = db.prepare(
            "SELECT id, created, model, tokens_prompt, tokens_completion, native_tokens_prompt, native_tokens_completion, total_cost FROM ai_generations ORDER BY created DESC LIMIT ?1 OFFSET ?2",
        )?;
        let iter = stmt.query_map(params![limit, offset], |row| {
            Ok(GenerationData {
                id: row.get(0)?,
                created: row.get(1)?,
                model: row.get(2)?,
                tokens_prompt: row.get(3)?,
                tokens_completion: row.get(4)?,
                native_tokens_prompt: row.get(5)?,
                native_tokens_completion: row.get(6)?,
                total_cost: row.get(7)?,
            })
        })?;

        iter.collect::<RusqliteResult<Vec<_>>>()
            .map_err(|e| e.into())
    }
}

#[tauri::command]
pub fn get_ai_usage_history(
    manager: State<AiUsageManager>,
    limit: u32,
    offset: u32,
) -> Result<Vec<GenerationData>, String> {
    manager
        .get_history(limit, offset)
        .map_err(|e| e.to_string())
}

async fn fetch_and_log_usage(
    open_router_request_id: String,
    api_key: String,
    manager: &AiUsageManager,
) -> Result<(), AppError> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://openrouter.ai/api/v1/generation?id={}",
            open_router_request_id
        ))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| AppError::Ai(e.to_string()))?;

    if response.status().is_success() {
        let generation_response: Value = response
            .json()
            .await
            .map_err(|e| AppError::Ai(e.to_string()))?;
        let generation_data: GenerationData =
            serde_json::from_value(generation_response["data"].clone())
                .map_err(|e| AppError::Ai(format!("Failed to parse generation data: {}", e)))?;
        manager.log_generation(&generation_data)?;
    } else {
        let error_text = response.text().await.unwrap_or_default();
        return Err(AppError::Ai(format!(
            "Failed to fetch usage data: {}",
            error_text
        )));
    }
    Ok(())
}

// --- Core Stream Command ---

#[tauri::command]
pub async fn ai_ask_stream(
    app_handle: AppHandle,
    manager: State<'_, AiUsageManager>,
    request_id: String,
    prompt: String,
    options: AskOptions,
) -> Result<(), String> {
    let api_key =
        match get_keyring_entry().and_then(|entry| entry.get_password().map_err(AppError::from)) {
            Ok(key) => key,
            Err(e) => return Err(e.to_string()),
        };

    let model_key = options.model.unwrap_or_else(|| "default".to_string());
    // For testing, use a free model if "default" is chosen.
    let model_id = options.model_mappings.get(&model_key).map_or_else(
        || "mistralai/mistral-7b-instruct:free".to_string(),
        |id| id.clone(),
    );

    let temperature = match options.creativity.as_deref() {
        Some("none") => 0.0,
        Some("low") => 0.4,
        Some("medium") => 0.7,
        Some("high") => 1.0,
        _ => 0.7,
    };

    let body = serde_json::json!({
        "model": model_id,
        "messages": [{"role": "user", "content": prompt}],
        "stream": true,
        "temperature": temperature,
    });

    let client = reqwest::Client::new();
    let res = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "http://localhost") // Required by OpenRouter
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let open_router_request_id = res
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    if !res.status().is_success() {
        let error_body = res.text().await.unwrap_or_else(|_| "Unknown error".into());
        return Err(format!("API Error: {}", error_body));
    }

    let mut stream = res.bytes_stream();
    let mut full_text = String::new();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;
        let lines = String::from_utf8_lossy(&chunk);

        for line in lines.split("\n\n").filter(|s| !s.is_empty()) {
            if line.starts_with("data: ") {
                let json_str = &line[6..];
                if json_str.trim() == "[DONE]" {
                    break;
                }
                if let Ok(json) = serde_json::from_str::<Value>(json_str) {
                    if let Some(delta) = json
                        .get("choices")
                        .and_then(|c| c.get(0))
                        .and_then(|c0| c0.get("delta"))
                    {
                        if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                            full_text.push_str(content);
                            app_handle
                                .emit(
                                    "ai-stream-chunk",
                                    StreamChunk {
                                        request_id: request_id.clone(),
                                        text: content.to_string(),
                                    },
                                )
                                .map_err(|e| e.to_string())?;
                        }
                    }
                }
            }
        }
    }

    app_handle
        .emit(
            "ai-stream-end",
            StreamEnd {
                request_id: request_id.clone(),
                full_text: full_text.clone(),
            },
        )
        .map_err(|e| e.to_string())?;

    if let Some(or_req_id) = open_router_request_id {
        let manager_clone = AiUsageManager {
            db: Mutex::new(Connection::open(manager.db.lock().unwrap().path().unwrap()).unwrap()),
        };
        tokio::spawn(async move {
            if let Err(e) = fetch_and_log_usage(or_req_id, api_key, &manager_clone).await {
                eprintln!("[AI Usage Tracking] Error: {}", e);
            }
        });
    }

    Ok(())
}
