use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::{Duration, SystemTime},
};

use freedesktop_file_parser::{parse, EntryType};
use rayon::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub name: String,
    pub comment: Option<String>,
    pub exec: Option<String>,
    pub icon_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AppCache {
    apps: Vec<App>,
    dir_mod_times: HashMap<PathBuf, SystemTime>,
}

fn get_app_dirs() -> Vec<PathBuf> {
    let mut app_dirs = vec![
        PathBuf::from("/usr/share/applications"),
        PathBuf::from("/usr/local/share/applications"),
    ];

    if let Ok(home_dir) = env::var("HOME") {
        let mut user_app_dir = PathBuf::from(home_dir);
        user_app_dir.push(".local/share/applications");
        app_dirs.push(user_app_dir);
    }
    app_dirs
}

fn get_cache_path() -> Result<PathBuf, String> {
    let cache_dir = env::var("XDG_CACHE_HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            env::var("HOME")
                .ok()
                .map(|home| PathBuf::from(home).join(".cache"))
        })
        .ok_or("Could not determine cache directory")?;
    
    let app_cache_dir = cache_dir.join("raycast-linux");
    fs::create_dir_all(&app_cache_dir).map_err(|e| e.to_string())?;
    Ok(app_cache_dir.join("apps.bincode"))
}

fn scan_and_cache_apps() -> Result<Vec<App>, String> {
    let app_dirs = get_app_dirs();
    let mut desktop_files = Vec::new();

    for dir in &app_dirs {
        if dir.exists() {
            desktop_files.extend(find_desktop_files(dir));
        }
    }

    let apps: Vec<App> = desktop_files
        .par_iter()
        .filter_map(|file_path| {
            let content = fs::read_to_string(file_path).ok()?;
            let desktop_file = parse(&content).ok()?;
            let entry = desktop_file.entry;

            if entry.hidden.unwrap_or(false) || entry.no_display.unwrap_or(false) {
                return None;
            }

            if let EntryType::Application(app_fields) = entry.entry_type {
                let app = App {
                    name: entry.name.default,
                    comment: entry.comment.map(|lc| lc.default),
                    exec: app_fields.exec,
                    icon_path: entry
                        .icon
                        .and_then(|ic| ic.get_icon_path())
                        .and_then(|p| p.to_str().map(String::from)),
                };
                if app.exec.is_some() && !app.name.is_empty() {
                    return Some(app);
                }
            }
            None
        })
        .collect();

    let mut unique_apps = Vec::new();
    let mut seen_app_names = HashSet::new();
    for app in apps {
        if seen_app_names.insert(app.name.clone()) {
            unique_apps.push(app);
        }
    }

    unique_apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    
    let dir_mod_times = app_dirs
        .into_iter()
        .filter_map(|dir| {
            let metadata = fs::metadata(&dir).ok()?;
            let mod_time = metadata.modified().ok()?;
            Some((dir, mod_time))
        })
        .collect();

    let cache_data = AppCache {
        apps: unique_apps.clone(),
        dir_mod_times,
    };

    if let Ok(cache_path) = get_cache_path() {
       let encoded = bincode::serde::encode_to_vec(&cache_data, bincode::config::standard()).map_err(|e| e.to_string())?;
       fs::write(cache_path, encoded).map_err(|e| e.to_string())?;
    }

    Ok(unique_apps)
}

fn find_desktop_files(path: &Path) -> Vec<PathBuf> {
    let mut desktop_files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                desktop_files.extend(find_desktop_files(&path));
            } else if path.extension().map_or(false, |ext| ext == "desktop") {
                desktop_files.push(path);
            }
        }
    }
    desktop_files
}

#[tauri::command]
fn get_installed_apps() -> Vec<App> {
    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(_) => return scan_and_cache_apps().unwrap_or_default(),
    };

    if let Ok(file_content) = fs::read(&cache_path) {
        if let Ok((cached_data, _)) = bincode::serde::decode_from_slice::<AppCache, _>(&file_content, bincode::config::standard()) {
            let is_stale = get_app_dirs().into_iter().any(|dir| {
                let current_mod_time = fs::metadata(&dir).ok().and_then(|m| m.modified().ok());
                let cached_mod_time = cached_data.dir_mod_times.get(&dir);
                
                match (current_mod_time, cached_mod_time) {
                    (Some(current), Some(cached)) => current != *cached,
                    _ => true, 
                }
            });

            if !is_stale {
                return cached_data.apps;
            }
        }
    }
    
    scan_and_cache_apps().unwrap_or_default()
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
        .setup(|_app| {
            thread::spawn(|| {
                loop {
                    thread::sleep(Duration::from_secs(300));
                    let _ = scan_and_cache_apps();
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}