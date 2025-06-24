mod app;
mod browser_extension;
mod cache;
mod clipboard;
mod clipboard_history;
mod desktop;
mod error;
mod extensions;
mod filesystem;
mod oauth;
mod quicklinks;
mod system;

use crate::{app::App, cache::AppCache};
use browser_extension::WsState;
use quicklinks::QuicklinkManager;
use selection::get_text;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Manager};

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
async fn show_hud(app: tauri::AppHandle, title: String) -> Result<(), String> {
    let hud_window = match app.get_webview_window("hud") {
        Some(window) => window,
        None => {
            tauri::WebviewWindowBuilder::new(&app, "hud", tauri::WebviewUrl::App("/hud".into()))
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .center()
                .min_inner_size(300.0, 80.0)
                .max_inner_size(300.0, 80.0)
                .inner_size(300.0, 80.0)
                .build()
                .map_err(|e| e.to_string())?
        }
    };

    let window_clone = hud_window.clone();
    window_clone.show().map_err(|e| e.to_string())?;
    window_clone
        .emit("hud-message", &title)
        .map_err(|e| e.to_string())?;
    window_clone
        .set_ignore_cursor_events(true)
        .map_err(|e| e.to_string())?;
    window_clone.set_focus().map_err(|e| e.to_string())?;

    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let _ = window_clone.hide();
    });

    Ok(())
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
        .plugin(tauri_plugin_http::init())
        .manage(WsState::default())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if args.len() > 1 && args[1].starts_with("raycast://") {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("deep-link", args[1].to_string());
                    window.show().unwrap();
                    window.set_focus().unwrap();
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
            show_hud,
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
            oauth::oauth_remove_tokens,
            clipboard_history::history_get_items,
            clipboard_history::history_get_item_content,
            clipboard_history::history_delete_item,
            clipboard_history::history_toggle_pin,
            clipboard_history::history_clear_all,
            clipboard_history::history_item_was_copied,
            quicklinks::create_quicklink,
            quicklinks::list_quicklinks,
            quicklinks::update_quicklink,
            quicklinks::delete_quicklink,
            quicklinks::execute_quicklink,
            system::get_applications,
            system::get_default_application,
            system::get_frontmost_application,
            system::show_in_finder,
            system::trash
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(browser_extension::run_server(app_handle));

            let app_handle_for_history = app.handle().clone();
            clipboard_history::init(app_handle_for_history);

            let quicklink_manager = QuicklinkManager::new(app.handle().clone())?;
            quicklink_manager.init_db()?;
            app.manage(quicklink_manager);

            setup_background_refresh();
            setup_global_shortcut(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}