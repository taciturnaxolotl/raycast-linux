pub mod engine;
pub mod input_manager;
pub mod manager;
pub mod types;

use tauri::{AppHandle, Manager, State};
use types::Snippet;

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