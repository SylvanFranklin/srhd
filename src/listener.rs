use rdev::{listen, Event};
use std::{env, fs};

pub fn srhd_process() {
    let config = Config::load();
    let mut keys: HeldKeys = Vec::new();

    let callback = move |event: Event| match event.event_type {
        rdev::EventType::KeyRelease(key) => {
            keys.retain(|&x| x != key);
        }
        rdev::EventType::KeyPress(key) => {
            keys.push(key);
            config.run(&keys);
        }
        _ => {}
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
