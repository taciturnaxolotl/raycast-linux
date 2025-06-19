use std::fs;
use std::io::{self, Cursor};
use std::path::PathBuf;

use tauri::Manager;

#[tauri::command]
pub async fn install_extension(
    app: tauri::AppHandle,
    download_url: String,
    slug: String,
) -> Result<(), String> {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .or_else(|_| Err("Failed to get app local data dir".to_string()))?;

    let plugins_dir = data_dir.join("plugins");
    let extension_dir = plugins_dir.join(&slug);

    if !plugins_dir.exists() {
        fs::create_dir_all(&plugins_dir).map_err(|e| e.to_string())?;
    }

    if extension_dir.exists() {
        fs::remove_dir_all(&extension_dir).map_err(|e| e.to_string())?;
    }

    let response = reqwest::get(&download_url)
        .await
        .map_err(|e| format!("Failed to download extension: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to download extension: status code {}",
            response.status()
        ));
    }

    let content = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response bytes: {}", e))?;

    let mut archive = zip::ZipArchive::new(Cursor::new(content)).map_err(|e| e.to_string())?;

    let prefix_to_strip = {
        let file_names: Vec<PathBuf> = archive.file_names().map(PathBuf::from).collect();

        if file_names.len() <= 1 {
            None
        } else {
            let first_path = &file_names[0];
            if let Some(first_component) = first_path.components().next() {
                if file_names
                    .iter()
                    .all(|path| path.starts_with(first_component))
                {
                    Some(PathBuf::from(first_component.as_os_str()))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;

        let enclosed_path = match file.enclosed_name() {
            Some(path) => path.to_path_buf(),
            None => continue,
        };

        let final_path_part = if let Some(ref prefix) = prefix_to_strip {
            enclosed_path
                .strip_prefix(prefix)
                .unwrap_or(&enclosed_path)
                .to_path_buf()
        } else {
            enclosed_path
        };

        if final_path_part.as_os_str().is_empty() {
            continue;
        }

        let outpath = extension_dir.join(final_path_part);

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))
                    .map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
} 