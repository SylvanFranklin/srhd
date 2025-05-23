use rdev::GrabError;

use crate::config::{Binding, Config, Key};
use std::sync::{Arc, Mutex};

pub struct HeldKeys {
    pub command: bool,
    pub control: bool,
    pub shift: bool,
    pub option: bool,
    pub function: bool,
    pub capslock: bool,
    pub key: Option<rdev::Key>,
}

impl HeldKeys {
    fn new() -> Self {
        HeldKeys {
            command: false,
            control: false,
            shift: false,
            option: false,
            function: false,
            capslock: false,
            key: None,
        }
    }

    /// Checks if the binding is pressed, clean this up
    pub fn binding_pressed(&self, binding: &Binding) -> bool {
        let test: HeldKeys = binding.into();
        if test.key != self.key {
            return false;
        }

        if test.command != self.command {
            return false;
        }

        if test.control != self.control {
            return false;
        }

        if test.shift != self.shift {
            return false;
        }

        if test.option != self.option {
            return false;
        }

        true
    }

    fn toggle(&mut self, key: rdev::Key, is: bool) {
        use rdev::Key::*;
        match key {
            ControlLeft | ControlRight => self.control = is,
            ShiftLeft | ShiftRight => self.shift = is,
            MetaLeft | MetaRight => self.command = is,
            Alt | AltGr => self.option = is,
            Function => self.function = is,
            CapsLock => self.capslock = is,
            other if is => self.key = Some(other),
            _ => self.key = None,
        };
    }
}

fn lock() {
    let lock_file = std::fs::File::create("/tmp/srhd.pid");
}

/// Starts the main event loop for the listener
pub fn srhd_process(debug: bool) {
    let config = Config::load();
    let keys = Arc::new(Mutex::new(HeldKeys::new()));
    use rdev::{grab, Event};

    let callback = move |event: Event| -> Option<Event> {
        let mut keys = keys.lock().unwrap();

        match event.event_type {
            rdev::EventType::KeyRelease(key) => {
                keys.toggle(key, false);
                return Some(event);
            }
            rdev::EventType::KeyPress(key) => {
                keys.toggle(key, true);
                if config.execute_command(&keys) {
                    return None;
                }

                if debug {
                    let db: Key = key.into();
                    println!("{}", db);
                }

                return Some(event);
            }
            _ => return Some(event),
        }
    };

    if let Err(error) = grab(callback) {
        match error {
            GrabError::EventTapError => {
                eprintln!("Failed to listen to keys, you may need to re-enable the accessibility settings on your terminal emulator");
            }
            _ => {
                eprintln!("Error: {:?}", error);
            }
        }
    }
}
