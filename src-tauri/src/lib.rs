mod app;
mod browser_extension;
mod cache;
mod clipboard;
mod desktop;
mod error;
mod extensions;
mod filesystem;
mod oauth;

use crate::{app::App, cache::AppCache};
use browser_extension::WsState;
use selection::get_text;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Manager};
use tauri_plugin_deep_link::DeepLinkExt;

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

fn setup_background_refresh() {
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(60));
        loop {
            AppCache::refresh_background();
            thread::sleep(Duration::from_secs(300));
        }
    });
}

fn setup_global_shortcut(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_global_shortcut::{
        Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
    };

    let spotlight_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
    let handle = app.handle().clone();

    println!("Spotlight shortcut: {:?}", spotlight_shortcut);

    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, event| {
                println!("Shortcut: {:?}, Event: {:?}", shortcut, event);
                if shortcut == &spotlight_shortcut && event.state() == ShortcutState::Pressed {
                    let spotlight_window = handle.get_webview_window("raycast-linux").unwrap();
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(WsState::default())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if args.len() > 1 && args[1].starts_with("raycast://") {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("deep-link", args[1].to_string());
                    window.show().unwrap();
                }

                return;
            }

            if let Some(window) = app.get_webview_window("main") {
                if let Ok(true) = window.is_visible() {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_installed_apps,
            launch_app,
            get_selected_text,
            filesystem::get_selected_finder_items,
            extensions::install_extension,
            browser_extension::browser_extension_check_connection,
            browser_extension::browser_extension_request,
            clipboard::clipboard_read_text,
            clipboard::clipboard_read,
            clipboard::clipboard_copy,
            clipboard::clipboard_paste,
            clipboard::clipboard_clear,
            oauth::oauth_set_tokens,
            oauth::oauth_get_tokens,
            oauth::oauth_remove_tokens
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(browser_extension::run_server(app_handle));

            setup_background_refresh();
            setup_global_shortcut(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
