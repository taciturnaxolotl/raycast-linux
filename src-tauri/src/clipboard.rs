use enigo::{Enigo, Key, Keyboard, Settings};
use std::{thread, time::Duration};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadResult {
	text: Option<String>,
	html: Option<String>,
	file: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardContent {
	text: Option<String>,
	html: Option<String>,
	file: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CopyOptions {
	concealed: Option<bool>,
}

#[tauri::command]
pub async fn clipboard_read_text(app: tauri::AppHandle) -> Result<ReadResult, String> {
	let clipboard = app.clipboard();
	let text = clipboard.read_text().ok();
	Ok(ReadResult {
		text,
		html: None,
		file: None
	})
}

#[tauri::command]
pub async fn clipboard_read(app: tauri::AppHandle) -> Result<ReadResult, String> {
	let clipboard = app.clipboard();
	let text = clipboard.read_text().ok();
	let html = None; // read_html is not supported by the plugin

	let file = if let Some(ref text_content) = text {
		if text_content.lines().count() == 1
			&& (text_content.starts_with('/') || text_content.starts_with("file://"))
		{
			Some(text_content.clone())
		} else {
			None
		}
	} else {
		None
	};

	Ok(ReadResult { text, html, file })
}

#[tauri::command]
pub async fn clipboard_copy(
	app: tauri::AppHandle,
	content: ClipboardContent,
	_options: Option<CopyOptions>
) -> Result<(), String> {
	let clipboard = app.clipboard();

	if let Some(file_path) = &content.file {
		clipboard
			.write_text(file_path.clone())
			.map_err(|e| e.to_string())?;
	} else if let Some(html) = &content.html {
		clipboard
			.write_html(html.clone(), content.text)
			.map_err(|e| e.to_string())?;
	} else if let Some(text) = &content.text {
		clipboard.write_text(text.clone()).map_err(|e| e.to_string())?;
	}

	Ok(())
}

#[tauri::command]
pub async fn clipboard_paste(
	app: tauri::AppHandle,
	content: ClipboardContent
) -> Result<(), String> {
	let clipboard = app.clipboard();
	let original_text = clipboard.read_text().ok();

	clipboard_copy(app.clone(), content, None).await?;

	thread::sleep(Duration::from_millis(100));

	let mut enigo = Enigo::new(&Settings::default()).map_err(|e| e.to_string())?;

	#[cfg(target_os = "macos")]
	{
		enigo.key(Key::Meta, enigo::Direction::Press).ok();
		enigo.key(Key::Unicode('v'), enigo::Direction::Click).ok();
		enigo.key(Key::Meta, enigo::Direction::Release).ok();
	}
	#[cfg(not(target_os = "macos"))]
	{
		enigo.key(Key::Control, enigo::Direction::Press).ok();
		enigo.key(Key::Unicode('v'), enigo::Direction::Click).ok();
		enigo.key(Key::Control, enigo::Direction::Release).ok();
	}

	thread::sleep(Duration::from_millis(100));

	if let Some(text) = original_text {
		clipboard.write_text(text).map_err(|e| e.to_string())?;
	} else {
		clipboard.clear().map_err(|e| e.to_string())?;
	}

	Ok(())
}

#[tauri::command]
pub async fn clipboard_clear(app: tauri::AppHandle) -> Result<(), String> {
	app.clipboard().clear().map_err(|e| e.to_string())
}