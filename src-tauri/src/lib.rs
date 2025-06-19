mod app;
mod cache;
mod desktop;
mod error;

use crate::{app::App, cache::AppCache};
#[cfg(target_os = "linux")]
use arboard;
use selection::get_text;
use std::fs;
use std::io::{self, Cursor};
#[cfg(target_os = "linux")]
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::Manager;
#[cfg(target_os = "linux")]
use url::Url;
#[cfg(target_os = "linux")]
use zbus;

#[derive(serde::Serialize, Clone, Debug)]
pub struct FileSystemItem {
    path: String,
}

#[tauri::command]
async fn get_selected_finder_items() -> Result<Vec<FileSystemItem>, String> {
    #[cfg(target_os = "macos")]
    {
        get_selected_finder_items_macos()
    }
    #[cfg(target_os = "windows")]
    {
        get_selected_finder_items_windows()
    }
    #[cfg(target_os = "linux")]
    {
        get_selected_finder_items_linux().await
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Err("Unsupported operating system".to_string())
    }
}

#[cfg(target_os = "macos")]
fn get_selected_finder_items_macos() -> Result<Vec<FileSystemItem>, String> {
    let script = r#"
        tell application "Finder"
            if not running then
                return ""
            end if
            try
                set theSelection to selection
                if theSelection is {} then
                    return ""
                end if
                set thePaths to {}
                repeat with i from 1 to count of theSelection
                    set end of thePaths to (POSIX path of (item i of theSelection as alias))
                end repeat
                return thePaths
            on error
                return ""
            end try
        end tell
    "#;

    let output = std::process::Command::new("osascript")
        .arg("-l")
        .arg("AppleScript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute osascript: {}", e))?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        if error_message.contains("Finder is not running") {
            return Ok(vec![]);
        }
        return Err(format!("osascript failed with error: {}", error_message));
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str.is_empty() {
        return Ok(vec![]);
    }

    let paths: Vec<FileSystemItem> = result_str
        .split(", ")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| FileSystemItem {
            path: p.to_string(),
        })
        .collect();

    Ok(paths)
}

#[cfg(target_os = "windows")]
fn get_selected_finder_items_windows() -> Result<Vec<FileSystemItem>, String> {
    let script = r#"
        Add-Type @"
            using System;
            using System.Runtime.InteropServices;
            public class Win32 {
                [DllImport("user32.dll")]
                public static extern IntPtr GetForegroundWindow();
            }
"@
        $foreground_hwnd = [Win32]::GetForegroundWindow()
        $shell = New-Object -ComObject Shell.Application
        $window = $shell.Windows() | Where-Object { $_.HWND -eq $foreground_hwnd } | Select-Object -First 1
        if ($window) {
            if ($window.FullName -like "*\explorer.exe") {
                $selection = $window.Document.SelectedItems()
                if ($selection) {
                    $paths = $selection | ForEach-Object { $_.Path }
                    if ($paths) {
                        return $paths -join [System.Environment]::NewLine
                    }
                }
            }
        }
        return ""
    "#;

    let output = std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to execute powershell: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "powershell failed with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str.is_empty() {
        return Ok(vec![]);
    }

    let paths: Vec<FileSystemItem> = result_str
        .lines()
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| FileSystemItem {
            path: p.to_string(),
        })
        .collect();

    Ok(paths)
}

#[cfg(target_os = "linux")]
async fn get_selected_finder_items_linux() -> Result<Vec<FileSystemItem>, String> {
    if let Ok(paths) = get_from_file_manager().await {
        if !paths.is_empty() {
            return Ok(paths);
        }
    }

    if let Ok(paths) = get_from_clipboard() {
        if !paths.is_empty() {
            return Ok(paths);
        }
    }

    Err("Could not determine selected files. Please copy them to your clipboard.".to_string())
}

#[cfg(target_os = "linux")]
async fn get_from_file_manager() -> Result<Vec<FileSystemItem>, String> {
    let connection = match zbus::Connection::session().await {
        Ok(c) => c,
        Err(_) => return Ok(vec![]),
    };

    let proxy = match zbus::Proxy::new(
        &connection,
        "org.freedesktop.FileManager1",
        "/org/freedesktop/FileManager1",
        "org.freedesktop.FileManager1",
    )
    .await
    {
        Ok(p) => p,
        Err(_) => return Ok(vec![]),
    };

    let fm_service: String = proxy.destination().to_string();
    if fm_service.is_empty() {
        return Ok(vec![]);
    }

    let fm_name = fm_service.split('.').last().unwrap_or_default();
    let window_interface = format!("org.{}.Window", fm_name);

    if fm_name != "nautilus" && fm_name != "nemo" {
        return Ok(vec![]);
    }

    let response = match proxy.call_method("GetWindows", &()).await {
        Ok(r) => r,
        Err(_) => return Ok(vec![]),
    };

    let body = response.body();
    let windows: Vec<zbus::zvariant::ObjectPath> = body.deserialize().unwrap_or_default();

    for window_path in windows.iter().rev() {
        let fm_service_ref = fm_service.as_str();
        let window_interface_ref = window_interface.as_str();
        let window_proxy = match zbus::Proxy::new(
            &connection,
            fm_service_ref,
            window_path,
            window_interface_ref,
        )
        .await
        {
            Ok(p) => p,
            Err(_) => continue,
        };
        if let Ok(is_active) = window_proxy.get_property::<bool>("Active").await {
            if is_active {
                if let Ok(uris) = window_proxy
                    .get_property::<Vec<String>>("SelectedUris")
                    .await
                {
                    let paths = uris
                        .iter()
                        .filter_map(|uri_str| Url::parse(uri_str).ok())
                        .filter_map(|url| url.to_file_path().ok())
                        .map(|path_buf| FileSystemItem {
                            path: path_buf.to_string_lossy().into_owned(),
                        })
                        .collect();
                    return Ok(paths);
                }
            }
        }
    }
    Ok(vec![])
}

#[cfg(target_os = "linux")]
fn get_from_clipboard() -> Result<Vec<FileSystemItem>, String> {
    let mut clipboard = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    if let Ok(text) = clipboard.get_text() {
        let paths: Vec<FileSystemItem> = text
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                if trimmed.starts_with("file://") {
                    Url::parse(trimmed).ok().and_then(|u| u.to_file_path().ok())
                } else {
                    Some(Path::new(trimmed).to_path_buf())
                }
            })
            .filter(|p| p.exists())
            .map(|p| FileSystemItem {
                path: p.to_string_lossy().to_string(),
            })
            .collect();
        return Ok(paths);
    }
    Ok(vec![])
}

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

#[tauri::command]
async fn install_extension(
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            if let Some(window) = app.get_webview_window("main") {
                if let Ok(true) = window.is_visible() {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_installed_apps,
            launch_app,
            get_selected_text,
            get_selected_finder_items,
            install_extension
        ])
        .setup(|app| {
            use tauri_plugin_global_shortcut::{
                Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
            };

            thread::spawn(|| {
                thread::sleep(Duration::from_secs(60));
                loop {
                    AppCache::refresh_background();
                    thread::sleep(Duration::from_secs(300));
                }
            });

            let spotlight_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);

            let handle = app.handle().clone();

            println!("Spotlight shortcut: {:?}", spotlight_shortcut);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |_app, shortcut, event| {
                        println!("Shortcut: {:?}, Event: {:?}", shortcut, event);
                        if shortcut == &spotlight_shortcut
                            && event.state() == ShortcutState::Pressed
                        {
                            let spotlight_window =
                                handle.get_webview_window("raycast-linux").unwrap();
                            println!("Spotlight window: {:?}", spotlight_window);
                            if spotlight_window.is_visible().unwrap_or(false) {
                                spotlight_window.hide().unwrap();
                            } else {
                                spotlight_window.show().unwrap();
                                spotlight_window.set_focus().unwrap();
                            }
                        }
                    })
                    .build(),
            )?;

            app.global_shortcut().register(spotlight_shortcut)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
