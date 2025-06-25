use crate::snippets::input_manager::{InputEvent, InputManager};
use crate::snippets::manager::SnippetManager;
use rdev::Key;
use std::sync::{Arc, Mutex};
use std::thread;

const BUFFER_SIZE: usize = 30;

pub struct ExpansionEngine {
    buffer: Arc<Mutex<String>>,
    snippet_manager: Arc<SnippetManager>,
    input_manager: Arc<dyn InputManager>,
}

fn key_to_char(key: &Key, is_shifted: bool) -> Option<char> {
    if let Key::Backspace = key {
        return Some('\u{8}');
    }
    if let Key::Return | Key::KpReturn = key {
        return Some('\n');
    }
    if let Key::Tab = key {
        return Some('\t');
    }

    let s = match key {
        Key::KeyA => "aA",
        Key::KeyB => "bB",
        Key::KeyC => "cC",
        Key::KeyD => "dD",
        Key::KeyE => "eE",
        Key::KeyF => "fF",
        Key::KeyG => "gG",
        Key::KeyH => "hH",
        Key::KeyI => "iI",
        Key::KeyJ => "jJ",
        Key::KeyK => "kK",
        Key::KeyL => "lL",
        Key::KeyM => "mM",
        Key::KeyN => "nN",
        Key::KeyO => "oO",
        Key::KeyP => "pP",
        Key::KeyQ => "qQ",
        Key::KeyR => "rR",
        Key::KeyS => "sS",
        Key::KeyT => "tT",
        Key::KeyU => "uU",
        Key::KeyV => "vV",
        Key::KeyW => "wW",
        Key::KeyX => "xX",
        Key::KeyY => "yY",
        Key::KeyZ => "zZ",
        Key::Num0 => "0)",
        Key::Num1 => "1!",
        Key::Num2 => "2@",
        Key::Num3 => "3#",
        Key::Num4 => "4$",
        Key::Num5 => "5%",
        Key::Num6 => "6^",
        Key::Num7 => "7&",
        Key::Num8 => "8*",
        Key::Num9 => "9(",
        Key::Space => "  ",
        Key::Slash => "/?",
        Key::Dot => ".>",
        Key::Comma => ",<",
        Key::Minus => "-_",
        Key::Equal => "=+",
        Key::LeftBracket => "[{",
        Key::RightBracket => "]}",
        Key::BackSlash => "\\|",
        Key::SemiColon => ";:",
        Key::Quote => "'\"",
        Key::BackQuote => "`~",
        _ => return None,
    };
    s.chars().nth(if is_shifted { 1 } else { 0 })
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