use crate::{app::App, desktop::DesktopFileManager, error::AppError};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Serialize, Deserialize)]
pub struct AppCache {
    apps: Vec<App>,
    dir_mod_times: HashMap<PathBuf, SystemTime>,
}

impl AppCache {
    pub fn get_cache_path() -> Result<PathBuf, AppError> {
        let cache_dir = env::var("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .or_else(|_| env::var("HOME").map(|home| PathBuf::from(home).join(".cache")))
            .map_err(|_| AppError::DirectoryNotFound)?;

        let app_cache_dir = cache_dir.join("raycast-linux");
        fs::create_dir_all(&app_cache_dir)?;
        Ok(app_cache_dir.join("apps.bincode"))
    }

    pub fn read_from_file(path: &Path) -> Result<AppCache, AppError> {
        let file_content = fs::read(path)?;
        let (decoded, _) =
            bincode::serde::decode_from_slice(&file_content, bincode::config::standard())?;
        Ok(decoded)
    }

    pub fn write_to_file(&self, path: &Path) -> Result<(), AppError> {
        let encoded = bincode::serde::encode_to_vec(self, bincode::config::standard())?;
        fs::write(path, encoded)?;
        Ok(())
    }

    pub fn is_stale(&self) -> bool {
        DesktopFileManager::get_app_directories()
            .into_iter()
            .any(|dir| {
                let current_mod_time = fs::metadata(&dir).ok().and_then(|m| m.modified().ok());
                let cached_mod_time = self.dir_mod_times.get(&dir);

                match (current_mod_time, cached_mod_time) {
                    (Some(current), Some(cached)) => current > *cached,
                    _ => true,
                }
            })
    }

    pub fn get_apps() -> Result<Vec<App>, AppError> {
        let cache_path = Self::get_cache_path()?;

        if let Ok(cached_data) = Self::read_from_file(&cache_path) {
            if !cached_data.is_stale() {
                return Ok(cached_data.apps);
            }
        }

        Self::refresh_and_get_apps()
    }

    pub fn refresh_and_get_apps() -> Result<Vec<App>, AppError> {
        let (apps, dir_mod_times) = DesktopFileManager::scan_and_parse_apps()?;
        let cache_data = AppCache {
            apps: apps.clone(),
            dir_mod_times,
        };

        if let Ok(cache_path) = Self::get_cache_path() {
            if let Err(e) = cache_data.write_to_file(&cache_path) {
                eprintln!("Failed to write to app cache: {:?}", e);
            }
        }

        Ok(apps)
    }

    pub fn refresh_background() {
        if let Err(e) = Self::refresh_and_get_apps() {
            eprintln!("Error refreshing app cache in background: {:?}", e);
        }
    }
} 