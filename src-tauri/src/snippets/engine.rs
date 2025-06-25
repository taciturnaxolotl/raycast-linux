use crate::snippets::input_manager::{InputEvent, InputManager};
use crate::snippets::manager::SnippetManager;
use arboard::Clipboard;
use chrono::Local;
use enigo::Key as EnigoKey;
use std::sync::{Arc, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 30;

struct ResolvedSnippet {
    content: String,
    cursor_pos: Option<usize>,
}

pub struct ExpansionEngine {
    buffer: Arc<Mutex<String>>,
    snippet_manager: Arc<SnippetManager>,
    input_manager: Arc<dyn InputManager>,
}

impl ExpansionEngine {
    pub fn new(snippet_manager: Arc<SnippetManager>, input_manager: Arc<dyn InputManager>) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(String::with_capacity(BUFFER_SIZE))),
            snippet_manager,
            input_manager,
        }
    }

    pub fn start_listening(&self) -> anyhow::Result<()> {
        let engine = Arc::new(self.clone_for_thread());
        self.input_manager.start_listening(Box::new(move |event| {
            engine.handle_key_press(event);
        }))?;
        Ok(())
    }

    fn clone_for_thread(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            snippet_manager: self.snippet_manager.clone(),
            input_manager: self.input_manager.clone(),
        }
    }

    fn handle_key_press(&self, event: InputEvent) {
        let InputEvent::KeyPress(ch) = event;
        let mut buffer = self.buffer.lock().unwrap();

        match ch {
            '\u{8}' => {
                buffer.pop();
            }
            '\n' | '\t' | '\u{1b}' => {
                buffer.clear();
            }
            ch if ch.is_control() => (),
            _ => {
                buffer.push(ch);
                if buffer.len() > BUFFER_SIZE {
                    buffer.remove(0);
                }
            }
        }

        if let Ok(snippets) = self.snippet_manager.list_snippets() {
            for snippet in snippets {
                if buffer.ends_with(&snippet.keyword) {
                    let (keyword, content) = (snippet.keyword.clone(), snippet.content.clone());
                    drop(buffer);
                    self.expand_snippet(&keyword, &content);
                    break;
                }
            }
        }
    }

    fn parse_and_resolve_placeholders(&self, raw_content: &str) -> ResolvedSnippet {
        let mut resolved_content = String::with_capacity(raw_content.len());
        let mut cursor_pos: Option<usize> = None;
        let mut last_end = 0;

        for (start, _) in raw_content.match_indices('{') {
            if start < last_end {
                continue;
            }
            if let Some(end) = raw_content[start..].find('}') {
                let placeholder = &raw_content[start + 1..start + end];

                resolved_content.push_str(&raw_content[last_end..start]);

                let replacement = match placeholder {
                    "cursor" => {
                        if cursor_pos.is_none() {
                            cursor_pos = Some(resolved_content.chars().count());
                        }
                        String::new()
                    }
                    "clipboard" => Clipboard::new()
                        .ok()
                        .and_then(|mut c| c.get_text().ok())
                        .unwrap_or_default(),
                    "date" => Local::now().format("%d %b %Y").to_string(),
                    "time" => Local::now().format("%H:%M").to_string(),
                    "datetime" => Local::now().format("%d %b %Y at %H:%M").to_string(),
                    "day" => Local::now().format("%A").to_string(),
                    _ => raw_content[start..start + end + 1].to_string(),
                };
                resolved_content.push_str(&replacement);
                last_end = start + end + 1;
            }
        }
        resolved_content.push_str(&raw_content[last_end..]);

        ResolvedSnippet {
            content: resolved_content,
            cursor_pos,
        }
    }

    fn expand_snippet(&self, keyword: &str, content: &str) {
        let mut backspaces = String::new();
        for _ in 0..keyword.len() {
            backspaces.push('\u{8}');
        }

        let resolved = self.parse_and_resolve_placeholders(content);
        let content_to_paste = resolved.content;

        let chars_to_move_left = if let Some(pos) = resolved.cursor_pos {
            content_to_paste.chars().count() - pos
        } else {
            0
        };

        let input_manager = self.input_manager.clone();

        thread::spawn(move || {
            if let Err(e) = input_manager.inject_text(&backspaces) {
                eprintln!("Failed to inject backspaces: {}", e);
            }
            thread::sleep(std::time::Duration::from_millis(50));
            if let Err(e) = input_manager.inject_text(&content_to_paste) {
                eprintln!("Failed to inject snippet content: {}", e);
            }

            if chars_to_move_left > 0 {
                thread::sleep(std::time::Duration::from_millis(50));
                if let Err(e) =
                    input_manager.inject_key_clicks(EnigoKey::LeftArrow, chars_to_move_left)
                {
                    eprintln!("Failed to inject cursor movement: {}", e);
                }
            }
        });

        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }
}
