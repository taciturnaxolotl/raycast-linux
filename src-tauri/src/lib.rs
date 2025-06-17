use std::{
    collections::HashSet, env, fs, path::{Path, PathBuf}, process::Command
};

use freedesktop_file_parser::{parse, EntryType};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct App {
    pub name: String,
    pub comment: Option<String>,
    pub exec: Option<String>,
    pub icon_path: Option<String>,
}

fn find_desktop_files(path: &Path) -> Vec<PathBuf> {
    let mut desktop_files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                desktop_files.extend(find_desktop_files(&path));
            } else if let Some(extension) = path.extension() {
                if extension == "desktop" {
                    desktop_files.push(path);
                }
            }
        }
    }
    desktop_files
}

#[tauri::command]
fn get_installed_apps() -> Vec<App> {
    let mut app_dirs = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
    ];

    if let Ok(home_dir) = env::var("HOME") {
        let mut user_app_dir = PathBuf::from(home_dir);
        user_app_dir.push(".local/share/applications");
        app_dirs.push(user_app_dir);
    }
    
    let mut apps = Vec::new();
    let mut seen_app_ids = HashSet::new();

    for dir in app_dirs {
        for file_path in find_desktop_files(&dir) {
            let content = fs::read_to_string(&file_path).unwrap();
            if let Ok(desktop_file) = parse(&content) {
                let entry = desktop_file.entry;

                if entry.hidden.unwrap_or(false) || entry.no_display.unwrap_or(false) {
                    continue;
                }

                if let EntryType::Application(app_fields) = entry.entry_type {
                    let app_id = file_path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                    if !seen_app_ids.contains(&app_id) {
                         let app = App {
                            name: entry.name.default,
                            comment: entry.comment.map(|lc| lc.default),
                            exec: app_fields.exec,
                            icon_path: entry.icon
                                .and_then(|ic| ic.get_icon_path())
                                .and_then(|p| p.to_str().map(String::from)),
                        };
                        
                        if app.exec.is_some() && !app.name.is_empty() {
                            apps.push(app);
                            seen_app_ids.insert(app_id);
                        }
                    }
                }
            }
        }
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_installed_apps, launch_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
