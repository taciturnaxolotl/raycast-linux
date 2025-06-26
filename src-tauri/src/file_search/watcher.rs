use super::{manager::FileSearchManager, types::IndexedFile};
use crate::error::AppError;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, DebouncedEvent};
use std::{
    env,
    path::PathBuf,
    time::{Duration, SystemTime},
};
use tauri::{AppHandle, Manager};

async fn handle_event(app_handle: AppHandle, debounced_event: DebouncedEvent) {
    let manager = app_handle.state::<FileSearchManager>();
    let path = &debounced_event.event.paths[0];

    if path.exists() {
        if let Ok(metadata) = path.metadata() {
            let file_type = if metadata.is_dir() {
                "directory".to_string()
            } else {
                "file".to_string()
            };
            let last_modified = metadata
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;

            let indexed_file = IndexedFile {
                path: path.to_string_lossy().to_string(),
                name: path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default(),
                parent_path: path
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default(),
                file_type,
                last_modified,
            };
            if let Err(e) = manager.add_file(&indexed_file) {
                eprintln!(
                    "Failed to add/update file in index: {:?}, path: {}",
                    e,
                    path.display()
                );
            }
        }
    } else if let Err(e) = manager.remove_file(&path.to_string_lossy()) {
        eprintln!(
            "Failed to remove file from index: {:?}, path: {}",
            e,
            path.display()
        );
    }
}

pub async fn start_watching(app_handle: AppHandle) -> Result<(), AppError> {
    let home_dir = env::var("HOME").map_err(|e| AppError::FileSearch(e.to_string()))?;
    let app_handle_clone = app_handle.clone();

    let mut debouncer = new_debouncer(
        Duration::from_secs(2),
        None,
        move |result: DebounceEventResult| {
            let app_handle_clone2 = app_handle_clone.clone();
            match result {
                Ok(events) => {
                    for event in events {
                        tauri::async_runtime::spawn(handle_event(app_handle_clone2.clone(), event));
                    }
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("watch error: {:?}", error);
                    }
                }
            }
        },
    )
    .map_err(|e| AppError::FileSearch(e.to_string()))?;

    debouncer
        .watcher()
        .watch(&PathBuf::from(&home_dir), RecursiveMode::Recursive)
        .map_err(|e| AppError::FileSearch(e.to_string()))?;

    debouncer
        .cache()
        .add_root(&PathBuf::from(&home_dir), RecursiveMode::Recursive);

    app_handle.manage(debouncer);

    Ok(())
}
