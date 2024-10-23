use std::sync::{Arc, Mutex};

use rdev::{listen, Event};

use crate::config::{Config, Mods};

pub struct HeldKeys {
    pub keys: Vec<rdev::Key>,
    pub mods: Vec<Mods>,
}

impl HeldKeys {
    fn new() -> Self {
        HeldKeys {
            keys: vec![],
            mods: vec![],
        }
    }

    fn remove(&mut self, key: rdev::Key) {
        use rdev::Key::*;
        match key {
            ControlLeft | ControlRight => self.mods.retain(|e| *e != Mods::Control),
            ShiftLeft | ShiftRight => self.mods.retain(|e| *e != Mods::Shift),
            Alt => self.mods.retain(|e| *e != Mods::Option),
            MetaLeft | MetaRight => self.mods.retain(|e| *e != Mods::Command),
            other => self.keys.retain(|e| *e != other),
        };
    }

    fn push(&mut self, key: rdev::Key) {
        use rdev::Key::*;
        match key {
            ControlLeft | ControlRight => self.mods.push(Mods::Control),
            ShiftLeft | ShiftRight => self.mods.push(Mods::Shift),
            Alt => self.mods.push(Mods::Option),
            MetaLeft | MetaRight => self.mods.push(Mods::Command),
            other => self.keys.push(other),
        };
    }
}
/// Starts the main event loop for the listener
/// TODO, tap into the keyboard state instead of storing it in a vector;
pub fn srhd_process() {
    let config = Config::load();
    let keys = Arc::new(Mutex::new(HeldKeys::new()));
    use rdev::{grab, Event, EventType, Key};

    let callback = move |event: Event| -> Option<Event> {
        let mut keys = keys.lock().unwrap();

        match event.event_type {
            rdev::EventType::KeyRelease(key) => {
                keys.remove(key);
                return Some(event);
            }
            rdev::EventType::KeyPress(key) => {
                keys.push(key);
                config.execute_commands(&keys);
                return Some(event);
            }
            _ => return Some(event),
        }
    };

    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}
