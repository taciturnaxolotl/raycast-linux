mod app;
mod cache;
mod desktop;
mod error;

use crate::{
    app::App,
    cache::AppCache,
};
use selection::get_text;
use std::{process::Command, thread, time::Duration};

#[tauri::command]
fn get_installed_apps() -> Vec<App> {
    match AppCache::get_apps() {
        Ok(apps) => apps,
        Err(e) => {
            eprintln!("Failed to get apps: {:?}", e);
            Vec::new()
        }
    }
}

#[tauri::command]
fn launch_app(exec: String) -> Result<(), String> {
    let exec_parts: Vec<&str> = exec.split_whitespace().collect();
    if exec_parts.is_empty() {
        return Err("Empty exec command".to_string());
    }

    let mut command = Command::new(exec_parts[0]);
    for arg in &exec_parts[1..] {
        if !arg.starts_with('%') {
            command.arg(arg);
        }
    }

    command
        .spawn()
        .map_err(|e| format!("Failed to launch app: {}", e))?;

    Ok(())
}

#[tauri::command]
fn get_selected_text() -> String {
    get_text()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_installed_apps,
            launch_app,
            get_selected_text
        ])
        .setup(|_app| {
            thread::spawn(|| {
                thread::sleep(Duration::from_secs(60));
                loop {
                    AppCache::refresh_background();
                    thread::sleep(Duration::from_secs(300));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}