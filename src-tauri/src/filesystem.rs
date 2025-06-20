#[derive(serde::Serialize, Clone, Debug)]
pub struct FileSystemItem {
    path: String,
}

#[tauri::command]
pub async fn get_selected_finder_items() -> Result<Vec<FileSystemItem>, String> {
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
async fn get_from_file_manager() -> Result<Vec<FileSystemItem>, String> {
    use url::Url;
    use zbus;

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
    use arboard;
    use std::path::Path;
    use url::Url;

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
