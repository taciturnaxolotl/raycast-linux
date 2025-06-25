pub mod engine;
pub mod input_manager;
pub mod manager;
pub mod types;

use crate::error::AppError;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use types::Snippet;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportSnippet {
    name: String,
    text: String,
    keyword: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    snippets_added: u32,
    duplicates_skipped: u32,
}

#[tauri::command]
pub fn create_snippet(
    app: AppHandle,
    name: String,
    keyword: String,
    content: String,
) -> Result<i64, String> {
    app.state::<manager::SnippetManager>()
        .create_snippet(name, keyword, content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_snippets(app: AppHandle) -> Result<Vec<Snippet>, String> {
    app.state::<manager::SnippetManager>()
        .list_snippets()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_snippet(
    app: AppHandle,
    id: i64,
    name: String,
    keyword: String,
    content: String,
) -> Result<(), String> {
    app.state::<manager::SnippetManager>()
        .update_snippet(id, name, keyword, content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_snippet(app: AppHandle, id: i64) -> Result<(), String> {
    app.state::<manager::SnippetManager>()
        .delete_snippet(id)
        .map_err(|e| e.to_string())
}
#[tauri::command]
pub fn import_snippets(app: AppHandle, json_content: String) -> Result<ImportResult, String> {
    let snippets: Vec<ImportSnippet> =
        serde_json::from_str(&json_content).map_err(|e| e.to_string())?;

    let manager = app.state::<manager::SnippetManager>();
    let mut snippets_added = 0;
    let mut duplicates_skipped = 0;

    for snippet in snippets {
        let keyword = snippet.keyword;

        match manager.create_snippet(snippet.name, keyword, snippet.text) {
            Ok(_) => snippets_added += 1,
            Err(AppError::Rusqlite(rusqlite::Error::SqliteFailure(e, Some(msg))))
                if e.code == rusqlite::ErrorCode::ConstraintViolation
                    && msg.contains("UNIQUE constraint failed: snippets.keyword") =>
            {
                duplicates_skipped += 1;
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(ImportResult {
        snippets_added,
        duplicates_skipped,
    })
}
