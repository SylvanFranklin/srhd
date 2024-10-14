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
            Alt => self.mods.retain(|e| *e != Mods::Alt),
            MetaLeft | MetaRight => self.mods.retain(|e| *e != Mods::Command),
            other => self.keys.retain(|e| *e != other),
        };
    }

    fn push(&mut self, key: rdev::Key) {
        use rdev::Key::*;
        match key {
            ControlLeft | ControlRight => self.mods.push(Mods::Control),
            ShiftLeft | ShiftRight => self.mods.push(Mods::Shift),
            Alt => self.mods.push(Mods::Alt),
            MetaLeft | MetaRight => self.mods.push(Mods::Command),
            other => self.keys.push(other),
        };
    }
}

pub fn srhd_process() {
    let config = Config::load();
    let mut keys: HeldKeys = HeldKeys::new();

    let callback = move |event: Event| match event.event_type {
        rdev::EventType::KeyRelease(key) => {
            keys.remove(key);
        }
        rdev::EventType::KeyPress(key) => {
            keys.push(key);
            config.execute_commands(&keys);
        }
        _ => {}
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
