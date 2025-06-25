use anyhow::Result;
use enigo::{Enigo, Keyboard};
use lazy_static::lazy_static;
use rdev::Key;
use std::sync::Mutex;
use std::thread;

lazy_static! {
    static ref ENIGO: Mutex<Enigo> = Mutex::new(Enigo::new(&enigo::Settings::default()).unwrap());
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPress(Key),
}

pub trait InputManager: Send + Sync {
    fn start_listening(&self, callback: Box<dyn Fn(InputEvent) + Send>) -> Result<()>;
    fn inject_text(&self, text: &str) -> Result<()>;
}

pub struct RdevInputManager;

impl RdevInputManager {
    pub fn new() -> Self {
        Self
    }
}

impl InputManager for RdevInputManager {
    fn start_listening(&self, callback: Box<dyn Fn(InputEvent) + Send>) -> Result<()> {
        thread::spawn(move || {
            let cb = move |event: rdev::Event| {
                if let rdev::EventType::KeyPress(key) = event.event_type {
                    callback(InputEvent::KeyPress(key));
                }
            };
            if let Err(error) = rdev::listen(cb) {
                eprintln!("rdev error: {:?}", error)
            }
        });
        Ok(())
    }

    fn inject_text(&self, text: &str) -> Result<()> {
        let mut enigo = ENIGO.lock().unwrap();
        enigo.text(text)?;
        Ok(())
    }
}

// this implementation for wayland, because wayland is a pain and rdev no worky
#[cfg(target_os = "linux")]
pub struct EvdevInputManager;

#[cfg(target_os = "linux")]
impl EvdevInputManager {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "linux")]
impl InputManager for EvdevInputManager {
    fn start_listening(&self, callback: Box<dyn Fn(InputEvent) + Send>) -> Result<()> {
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

        for mut device in devices {
            thread::spawn(move || {
                loop {
                    match device.fetch_events() {
                        Ok(events) => {
                            for ev in events {
                                if ev.event_type() == evdev::EventType::KEY && ev.value() == 1 {
                                    println!("[evdev] Raw key press: code={}", ev.code());
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error fetching evdev events: {}", e);
                            break;
                        }
                    }
                }
            });
        }

        Ok(())
    }

    fn inject_text(&self, _text: &str) -> Result<()> {
        unimplemented!()
    }
}

pub fn key_to_char(key: &Key) -> Option<char> {
    match key {
        Key::KeyA => Some('a'),
        Key::KeyB => Some('b'),
        Key::KeyC => Some('c'),
        Key::KeyD => Some('d'),
        Key::KeyE => Some('e'),
        Key::KeyF => Some('f'),
        Key::KeyG => Some('g'),
        Key::KeyH => Some('h'),
        Key::KeyI => Some('i'),
        Key::KeyJ => Some('j'),
        Key::KeyK => Some('k'),
        Key::KeyL => Some('l'),
        Key::KeyM => Some('m'),
        Key::KeyN => Some('n'),
        Key::KeyO => Some('o'),
        Key::KeyP => Some('p'),
        Key::KeyQ => Some('q'),
        Key::KeyR => Some('r'),
        Key::KeyS => Some('s'),
        Key::KeyT => Some('t'),
        Key::KeyU => Some('u'),
        Key::KeyV => Some('v'),
        Key::KeyW => Some('w'),
        Key::KeyX => Some('x'),
        Key::KeyY => Some('y'),
        Key::KeyZ => Some('z'),
        Key::Num0 => Some('0'),
        Key::Num1 => Some('1'),
        Key::Num2 => Some('2'),
        Key::Num3 => Some('3'),
        Key::Num4 => Some('4'),
        Key::Num5 => Some('5'),
        Key::Num6 => Some('6'),
        Key::Num7 => Some('7'),
        Key::Num8 => Some('8'),
        Key::Num9 => Some('9'),
        Key::Space => Some(' '),
        Key::Slash => Some('/'),
        Key::Dot => Some('.'),
        Key::Comma => Some(','),
        Key::Minus => Some('-'),
        Key::Equal => Some('='),
        Key::LeftBracket => Some('['),
        Key::RightBracket => Some(']'),
        Key::BackSlash => Some('\\'),
        Key::SemiColon => Some(';'),
        Key::Quote => Some('\''),
        Key::BackQuote => Some('`'),
        _ => None,
    }
}