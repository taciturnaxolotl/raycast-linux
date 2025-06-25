use anyhow::Result;
use enigo::{Enigo, Key as EnigoKey, Keyboard};
use lazy_static::lazy_static;
use rdev::Key;
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(target_os = "linux")]
use evdev::{uinput::VirtualDevice, KeyCode};
#[cfg(target_os = "linux")]
use std::collections::HashSet;
#[cfg(target_os = "linux")]
use std::time::Duration;
#[cfg(target_os = "linux")]
use xkbcommon::xkb;

lazy_static! {
    static ref ENIGO: Mutex<Enigo> = Mutex::new(Enigo::new(&enigo::Settings::default()).unwrap());
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress(char),
}

pub trait InputManager: Send + Sync {
    fn start_listening(&self, callback: Box<dyn Fn(InputEvent) + Send + Sync>) -> Result<()>;
    fn inject_text(&self, text: &str) -> Result<()>;
    fn inject_key_clicks(&self, key: EnigoKey, count: usize) -> Result<()>;
}

pub struct RdevInputManager;

impl RdevInputManager {
    pub fn new() -> Self {
        Self
    }
}

impl InputManager for RdevInputManager {
    fn start_listening(&self, callback: Box<dyn Fn(InputEvent) + Send + Sync>) -> Result<()> {
        let shift_pressed = Arc::new(Mutex::new(false));
        let callback = Arc::new(callback);

        let shift_clone_press = shift_pressed.clone();
        let shift_clone_release = shift_pressed.clone();
        let callback_clone = callback.clone();

        thread::spawn(move || {
            let cb = move |event: rdev::Event| match event.event_type {
                    rdev::EventType::KeyPress(key) => {
                        if key == Key::ShiftLeft || key == Key::ShiftRight {
                            *shift_clone_press.lock().unwrap() = true;
                        }
                        let is_shifted = *shift_clone_press.lock().unwrap();
                        if let Some(ch) = key_to_char(&key, is_shifted) {
                            callback_clone(InputEvent::KeyPress(ch));
                        }
                    }
                    rdev::EventType::KeyRelease(key) => {
                        if key == Key::ShiftLeft || key == Key::ShiftRight {
                            *shift_clone_release.lock().unwrap() = false;
                        }
                    }
                    _ => (),
            };
            if let Err(error) = rdev::listen(cb) {
                eprintln!("rdev error: {:?}", error)
            }
        });
        Ok(())
    }

    fn inject_text(&self, text: &str) -> Result<()> {
        let mut enigo = ENIGO.lock().unwrap();
        let mut buffer = String::new();

        for c in text.chars() {
            if c == '\u{8}' {
                if !buffer.is_empty() {
                    enigo.text(&buffer)?;
                    buffer.clear();
                }
                enigo.key(enigo::Key::Backspace, enigo::Direction::Click)?;
            } else {
                buffer.push(c);
            }
        }

        if !buffer.is_empty() {
            enigo.text(&buffer)?;
        }

        Ok(())
    }

    fn inject_key_clicks(&self, key: EnigoKey, count: usize) -> Result<()> {
        let mut enigo = ENIGO.lock().unwrap();
        for _ in 0..count {
            enigo.key(key, enigo::Direction::Click)?;
        }
        Ok(())
    }
}

// this implementation for wayland, because wayland is a pain and rdev no worky
#[cfg(target_os = "linux")]
pub struct EvdevInputManager {
    virtual_device: Mutex<VirtualDevice>,
}

#[cfg(target_os = "linux")]
impl EvdevInputManager {
    pub fn new() -> Result<Self> {
        let mut key_codes = HashSet::new();
        key_codes.extend([
            KeyCode::KEY_LEFTSHIFT,
            KeyCode::KEY_ENTER,
            KeyCode::KEY_TAB,
            KeyCode::KEY_SPACE,
            KeyCode::KEY_BACKSPACE,
            KeyCode::KEY_LEFT,
        ]);

        let text: &str =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-=[]{}\\|;:'\",./<>?`~";
        for ch in text.chars() {
            if let Some((key, _)) = Self::char_to_keycode_static(ch) {
                key_codes.insert(key);
            }
        }

        let mut attribute_set = evdev::AttributeSet::new();
        for key in key_codes {
            attribute_set.insert(key);
        }

        let uinput_device = evdev::uinput::VirtualDevice::builder()?
            .name("Global Automata Text Injection")
            .with_keys(&attribute_set)?
            .build()?;

        Ok(Self {
            virtual_device: Mutex::new(uinput_device),
        })
    }

    fn char_to_keycode_static(c: char) -> Option<(KeyCode, bool)> {
        let (key, shift) = match c {
            'a' => (KeyCode::KEY_A, false),
            'b' => (KeyCode::KEY_B, false),
            'c' => (KeyCode::KEY_C, false),
            'd' => (KeyCode::KEY_D, false),
            'e' => (KeyCode::KEY_E, false),
            'f' => (KeyCode::KEY_F, false),
            'g' => (KeyCode::KEY_G, false),
            'h' => (KeyCode::KEY_H, false),
            'i' => (KeyCode::KEY_I, false),
            'j' => (KeyCode::KEY_J, false),
            'k' => (KeyCode::KEY_K, false),
            'l' => (KeyCode::KEY_L, false),
            'm' => (KeyCode::KEY_M, false),
            'n' => (KeyCode::KEY_N, false),
            'o' => (KeyCode::KEY_O, false),
            'p' => (KeyCode::KEY_P, false),
            'q' => (KeyCode::KEY_Q, false),
            'r' => (KeyCode::KEY_R, false),
            's' => (KeyCode::KEY_S, false),
            't' => (KeyCode::KEY_T, false),
            'u' => (KeyCode::KEY_U, false),
            'v' => (KeyCode::KEY_V, false),
            'w' => (KeyCode::KEY_W, false),
            'x' => (KeyCode::KEY_X, false),
            'y' => (KeyCode::KEY_Y, false),
            'z' => (KeyCode::KEY_Z, false),
            'A' => (KeyCode::KEY_A, true),
            'B' => (KeyCode::KEY_B, true),
            'C' => (KeyCode::KEY_C, true),
            'D' => (KeyCode::KEY_D, true),
            'E' => (KeyCode::KEY_E, true),
            'F' => (KeyCode::KEY_F, true),
            'G' => (KeyCode::KEY_G, true),
            'H' => (KeyCode::KEY_H, true),
            'I' => (KeyCode::KEY_I, true),
            'J' => (KeyCode::KEY_J, true),
            'K' => (KeyCode::KEY_K, true),
            'L' => (KeyCode::KEY_L, true),
            'M' => (KeyCode::KEY_M, true),
            'N' => (KeyCode::KEY_N, true),
            'O' => (KeyCode::KEY_O, true),
            'P' => (KeyCode::KEY_P, true),
            'Q' => (KeyCode::KEY_Q, true),
            'R' => (KeyCode::KEY_R, true),
            'S' => (KeyCode::KEY_S, true),
            'T' => (KeyCode::KEY_T, true),
            'U' => (KeyCode::KEY_U, true),
            'V' => (KeyCode::KEY_V, true),
            'W' => (KeyCode::KEY_W, true),
            'X' => (KeyCode::KEY_X, true),
            'Y' => (KeyCode::KEY_Y, true),
            'Z' => (KeyCode::KEY_Z, true),
            '1' => (KeyCode::KEY_1, false),
            '2' => (KeyCode::KEY_2, false),
            '3' => (KeyCode::KEY_3, false),
            '4' => (KeyCode::KEY_4, false),
            '5' => (KeyCode::KEY_5, false),
            '6' => (KeyCode::KEY_6, false),
            '7' => (KeyCode::KEY_7, false),
            '8' => (KeyCode::KEY_8, false),
            '9' => (KeyCode::KEY_9, false),
            '0' => (KeyCode::KEY_0, false),
            '!' => (KeyCode::KEY_1, true),
            '@' => (KeyCode::KEY_2, true),
            '#' => (KeyCode::KEY_3, true),
            '$' => (KeyCode::KEY_4, true),
            '%' => (KeyCode::KEY_5, true),
            '^' => (KeyCode::KEY_6, true),
            '&' => (KeyCode::KEY_7, true),
            '*' => (KeyCode::KEY_8, true),
            '(' => (KeyCode::KEY_9, true),
            ')' => (KeyCode::KEY_0, true),
            '-' => (KeyCode::KEY_MINUS, false),
            '_' => (KeyCode::KEY_MINUS, true),
            '=' => (KeyCode::KEY_EQUAL, false),
            '+' => (KeyCode::KEY_EQUAL, true),
            '[' => (KeyCode::KEY_LEFTBRACE, false),
            '{' => (KeyCode::KEY_LEFTBRACE, true),
            ']' => (KeyCode::KEY_RIGHTBRACE, false),
            '}' => (KeyCode::KEY_RIGHTBRACE, true),
            '\\' => (KeyCode::KEY_BACKSLASH, false),
            '|' => (KeyCode::KEY_BACKSLASH, true),
            ';' => (KeyCode::KEY_SEMICOLON, false),
            ':' => (KeyCode::KEY_SEMICOLON, true),
            '\'' => (KeyCode::KEY_APOSTROPHE, false),
            '"' => (KeyCode::KEY_APOSTROPHE, true),
            ',' => (KeyCode::KEY_COMMA, false),
            '<' => (KeyCode::KEY_COMMA, true),
            '.' => (KeyCode::KEY_DOT, false),
            '>' => (KeyCode::KEY_DOT, true),
            '/' => (KeyCode::KEY_SLASH, false),
            '?' => (KeyCode::KEY_SLASH, true),
            '`' => (KeyCode::KEY_GRAVE, false),
            '~' => (KeyCode::KEY_GRAVE, true),
            ' ' => (KeyCode::KEY_SPACE, false),
            '\n' => (KeyCode::KEY_ENTER, false),
            '\t' => (KeyCode::KEY_TAB, false),
            _ => return None,
        };
        Some((key, shift))
    }

    fn inject_char(&self, device: &mut VirtualDevice, c: char) -> Result<()> {
        let (key, shift) = match Self::char_to_keycode_static(c) {
            Some(val) => val,
            None => return Ok(()),
        };

        let syn = evdev::InputEvent::new(
            evdev::EventType::SYNCHRONIZATION.0,
            evdev::SynchronizationCode::SYN_REPORT.0,
            0,
        );

        if shift {
            device.emit(&[
                evdev::InputEvent::new(evdev::EventType::KEY.0, KeyCode::KEY_LEFTSHIFT.0, 1),
                syn.clone(),
            ])?;
        }

        device.emit(&[
            evdev::InputEvent::new(evdev::EventType::KEY.0, key.0, 1),
            syn.clone(),
        ])?;
        device.emit(&[
            evdev::InputEvent::new(evdev::EventType::KEY.0, key.0, 0),
            syn.clone(),
        ])?;

        if shift {
            device.emit(&[
                evdev::InputEvent::new(evdev::EventType::KEY.0, KeyCode::KEY_LEFTSHIFT.0, 0),
                syn.clone(),
            ])?;
        }

        thread::sleep(Duration::from_millis(10));
        Ok(())
    }

    fn inject_key_click(&self, device: &mut VirtualDevice, key: KeyCode) -> Result<()> {
        let syn = evdev::InputEvent::new(
            evdev::EventType::SYNCHRONIZATION.0,
            evdev::SynchronizationCode::SYN_REPORT.0,
            0,
        );
        device.emit(&[
            evdev::InputEvent::new(evdev::EventType::KEY.0, key.0, 1),
            syn.clone(),
        ])?;
        device.emit(&[
            evdev::InputEvent::new(evdev::EventType::KEY.0, key.0, 0),
            syn.clone(),
        ])?;

        thread::sleep(Duration::from_millis(10));
        Ok(())
    }

    fn enigo_to_evdev(key: EnigoKey) -> Option<KeyCode> {
        match key {
            EnigoKey::LeftArrow => Some(KeyCode::KEY_LEFT),
            _ => None,
        }
    }
}

#[cfg(target_os = "linux")]
impl InputManager for EvdevInputManager {
    fn start_listening(&self, callback: Box<dyn Fn(InputEvent) + Send + Sync>) -> Result<()> {
        let devices = evdev::enumerate()
            .map(|t| t.1)
            .filter(|d| {
                d.supported_keys()
                    .map_or(false, |keys| keys.contains(evdev::KeyCode::KEY_ENTER))
            })
            .collect::<Vec<_>>();

        if devices.is_empty() {
            return Err(anyhow::anyhow!(
                "No keyboard devices found. Check permissions for /dev/input/*."
            ));
        }

        let callback = Arc::new(callback);

        for mut device in devices {
            let callback = Arc::clone(&callback);
            let device_name = device.name().unwrap_or("Unnamed Device").to_string();

            thread::spawn(move || {
                let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
                let keymap = xkb::Keymap::new_from_names(
                    &context,
                    "",
                    "",
                    "",
                    "",
                    None,
                    xkb::KEYMAP_COMPILE_NO_FLAGS,
                )
                .expect("Failed to create xkb keymap");
                let mut xkb_state = xkb::State::new(&keymap);

                loop {
                    match device.fetch_events() {
                        Ok(events) => {
                            for ev in events {
                                if ev.event_type() != evdev::EventType::KEY {
                                    continue;
                                }

                                // evdev keycodes are offset by 8 from X11 keycodes
                                let keycode = ev.code() + 8;
                                let direction = match ev.value() {
                                    0 => xkb::KeyDirection::Up,
                                    1 => xkb::KeyDirection::Down,
                                    _ => continue,
                                };

                                match direction {
                                    xkb::KeyDirection::Down => {
                                        xkb_state.update_key(keycode.into(), direction);
                                        let keysym = xkb_state.key_get_one_sym(keycode.into());
                                        if keysym == xkb::keysyms::KEY_BackSpace.into() {
                                            callback(InputEvent::KeyPress('\u{8}'));
                                        } else {
                                            let utf8_str = xkb_state.key_get_utf8(keycode.into());
                                            if !utf8_str.is_empty() {
                                                for ch in utf8_str.chars() {
                                                    callback(InputEvent::KeyPress(ch));
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        xkb_state.update_key(keycode.into(), direction);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if e.kind() != std::io::ErrorKind::WouldBlock {
                                eprintln!(
                                    "Error fetching evdev events for device \"{}\": {}",
                                    device_name, e
                                );
                                break;
                            }
                        }
                    }
                }
            });
        }

        Ok(())
    }

    fn inject_text(&self, text: &str) -> Result<()> {
        let mut device = self.virtual_device.lock().unwrap();
        for ch in text.chars() {
            if ch == '\u{8}' {
                self.inject_key_click(&mut *device, KeyCode::KEY_BACKSPACE)?;
            } else {
                self.inject_char(&mut *device, ch)?;
            }
        }
        Ok(())
    }

    fn inject_key_clicks(&self, key: EnigoKey, count: usize) -> Result<()> {
        if let Some(keycode) = Self::enigo_to_evdev(key) {
            let mut device = self.virtual_device.lock().unwrap();
            for _ in 0..count {
                self.inject_key_click(&mut *device, keycode)?;
            }
        }
        Ok(())
    }
}

pub fn key_to_char(key: &Key, is_shifted: bool) -> Option<char> {
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