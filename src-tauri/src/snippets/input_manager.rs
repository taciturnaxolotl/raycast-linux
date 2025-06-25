use anyhow::Result;
use rdev::Key;
use std::thread;

#[derive(Debug)]
pub enum InputEvent {
    KeyPress(Key),
}

pub trait InputManager {
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

    fn inject_text(&self, _text: &str) -> Result<()> {
        unimplemented!()
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
            let cb = callback.as_ref();
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