use super::{manager::FileSearchManager, types::IndexedFile};
use std::{env, time::SystemTime};
use tauri::{AppHandle, Manager};
use walkdir::{DirEntry, WalkDir};

pub async fn build_initial_index(app_handle: AppHandle) {
    println!("Starting initial file index build.");
    let manager = app_handle.state::<FileSearchManager>();
    let home_dir = match env::var("HOME") {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get home directory: {}", e);
            return;
        }
    };

    let walker = WalkDir::new(home_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e) && !is_excluded(e)) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("Error walking directory: {}", e);
                continue;
            }
        };

        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(_) => continue,
        };

        let last_modified_secs = metadata
            .modified()
            .unwrap_or(SystemTime::UNIX_EPOCH)
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        if let Ok(Some(indexed_time)) = manager.get_file_last_modified(&path.to_string_lossy()) {
            if indexed_time >= last_modified_secs {
                if path.is_dir() {
                    // continue to walk children
                } else {
                    // skip this file
                    continue;
                }
            }
        }

        let file_type = if metadata.is_dir() {
            "directory".to_string()
        } else if metadata.is_file() {
            "file".to_string()
        } else {
            continue;
        };

        let indexed_file = IndexedFile {
            path: path.to_string_lossy().to_string(),
            name: entry.file_name().to_string_lossy().to_string(),
            parent_path: path
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            file_type,
            last_modified: last_modified_secs,
        };

        if let Err(e) = manager.add_file(&indexed_file) {
            eprintln!("Failed to add file to index: {:?}", e);
        }
    }
    println!("Finished initial file index build.");
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_excluded(entry: &DirEntry) -> bool {
    let path = entry.path();
    let excluded_dirs = [
        "node_modules",
        ".git",
        "target",
        ".vscode",
        ".idea",
        "__pycache__",
        ".cache",
        "Library",
        "Application Support",
        "AppData",
    ];
    path.components().any(|component| {
        excluded_dirs
            .iter()
            .any(|&excluded| component.as_os_str() == excluded)
    })
}
