use crate::snippets::input_manager::{InputEvent, InputManager};
use crate::snippets::manager::SnippetManager;
use std::sync::{Arc, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 30;

pub struct ExpansionEngine {
    buffer: Arc<Mutex<String>>,
    snippet_manager: Arc<SnippetManager>,
    input_manager: Arc<dyn InputManager>,
}

impl ExpansionEngine {
    pub fn new(
        snippet_manager: Arc<SnippetManager>,
        input_manager: Arc<dyn InputManager>,
    ) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(String::with_capacity(BUFFER_SIZE))),
            snippet_manager,
            input_manager,
        }
    }

    pub fn start_listening(&self) -> anyhow::Result<()> {
        let engine = Arc::new(self.clone_for_thread());
        self.input_manager
            .start_listening(Box::new(move |event| {
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

    fn expand_snippet(&self, keyword: &str, content: &str) {
        let mut backspaces = String::new();
        for _ in 0..keyword.len() {
            backspaces.push('\u{8}');
        }

        let input_manager = self.input_manager.clone();
        let content_to_paste = content.to_string();

        thread::spawn(move || {
            if let Err(e) = input_manager.inject_text(&backspaces) {
                eprintln!("Failed to inject backspaces: {}", e);
            }
            thread::sleep(std::time::Duration::from_millis(50));
            if let Err(e) = input_manager.inject_text(&content_to_paste) {
                eprintln!("Failed to inject snippet content: {}", e);
            }
        });

        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }
}