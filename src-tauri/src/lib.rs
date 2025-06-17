use freedesktop_file_parser::{parse, EntryType};
use rayon::prelude::*;
use selection::get_text;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    env, fs,
    io,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::{Duration, SystemTime},
};

#[derive(Debug)]
enum CacheError {
    Io,
    Bincode,
    DirectoryNotFound,
}

impl From<io::Error> for CacheError {
    fn from(_: io::Error) -> Self {
        CacheError::Io
    }
}

impl From<bincode::error::DecodeError> for CacheError {
    fn from(_: bincode::error::DecodeError) -> Self {
        CacheError::Bincode
    }
}

impl From<bincode::error::EncodeError> for CacheError {
    fn from(_: bincode::error::EncodeError) -> Self {
        CacheError::Bincode
    }
}

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
        app_dirs.push(PathBuf::from(home_dir).join(".local/share/applications"));
    }
    app_dirs
}

fn get_cache_path() -> Result<PathBuf, CacheError> {
    let cache_dir = env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|_| env::var("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .map_err(|_| CacheError::DirectoryNotFound)?;

    let app_cache_dir = cache_dir.join("raycast-linux");
    fs::create_dir_all(&app_cache_dir)?;
    Ok(app_cache_dir.join("apps.bincode"))
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

fn scan_and_parse_apps() -> Result<(Vec<App>, HashMap<PathBuf, SystemTime>), CacheError> {
    let app_dirs = get_app_dirs();
    let desktop_files: Vec<PathBuf> = app_dirs
        .iter()
        .filter(|dir| dir.exists())
        .flat_map(|dir| find_desktop_files(dir))
        .collect();

    let apps: Vec<App> = desktop_files
        .par_iter()
        .filter_map(|file_path| {
            let content = fs::read_to_string(file_path).ok()?;
            let desktop_file = parse(&content).ok()?;

            if desktop_file.entry.hidden.unwrap_or(false)
                || desktop_file.entry.no_display.unwrap_or(false)
            {
                return None;
            }

            if let EntryType::Application(app_fields) = desktop_file.entry.entry_type {
                if app_fields.exec.is_some() && !desktop_file.entry.name.default.is_empty() {
                    return Some(App {
                        name: desktop_file.entry.name.default,
                        comment: desktop_file.entry.comment.map(|lc| lc.default),
                        exec: app_fields.exec,
                        icon_path: desktop_file
                            .entry
                            .icon
                            .and_then(|ic| ic.get_icon_path())
                            .and_then(|p| p.to_str().map(String::from)),
                    });
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
            fs::metadata(&dir)
                .and_then(|m| m.modified())
                .ok()
                .map(|mod_time| (dir, mod_time))
        })
        .collect();

    Ok((unique_apps, dir_mod_times))
}

fn read_cache(path: &Path) -> Result<AppCache, CacheError> {
    let file_content = fs::read(path)?;
    let (decoded, _) = bincode::serde::decode_from_slice(&file_content, bincode::config::standard())?;
    Ok(decoded)
}

fn write_cache(path: &Path, cache: &AppCache) -> Result<(), CacheError> {
    let encoded = bincode::serde::encode_to_vec(cache, bincode::config::standard())?;
    fs::write(path, encoded)?;
    Ok(())
}

fn refresh_app_cache() {
    if let (Ok(cache_path), Ok((apps, dir_mod_times))) =
        (get_cache_path(), scan_and_parse_apps())
    {
        let cache_data = AppCache {
            apps,
            dir_mod_times,
        };
        if let Err(e) = write_cache(&cache_path, &cache_data) {
            eprintln!("Error refreshing app cache in background: {:?}", e);
        }
    }
}

#[tauri::command]
fn get_installed_apps() -> Vec<App> {
    let cache_path = match get_cache_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Could not get cache path: {:?}. Falling back to scan.", e);
            return scan_and_parse_apps().map_or_else(|_| Vec::new(), |(apps, _)| apps);
        }
    };

    if let Ok(cached_data) = read_cache(&cache_path) {
        let is_stale = get_app_dirs().into_iter().any(|dir| {
            let current_mod_time = fs::metadata(&dir).ok().and_then(|m| m.modified().ok());
            let cached_mod_time = cached_data.dir_mod_times.get(&dir);
            
            match (current_mod_time, cached_mod_time) {
                (Some(current), Some(cached)) => current > *cached,
                _ => true,
            }
        });

        if !is_stale {
            return cached_data.apps;
        }
    }
    
    match scan_and_parse_apps() {
        Ok((apps, dir_mod_times)) => {
            let cache_data = AppCache { apps: apps.clone(), dir_mod_times };
            if let Err(e) = write_cache(&cache_path, &cache_data) {
                eprintln!("Failed to write to app cache: {:?}", e);
            }
            apps
        }
        Err(e) => {
            eprintln!("Failed to scan and parse apps: {:?}", e);
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
                    refresh_app_cache();
                    thread::sleep(Duration::from_secs(300));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}