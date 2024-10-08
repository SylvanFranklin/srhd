use std::{env, fs};

pub type HeldKeys = Vec<rdev::Key>;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Binding {
    key: rdev::Key,
    command: String,
    mods: Vec<rdev::Key>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    binding: Vec<Binding>,
}

// methods, load, create_new, run
impl Config {
    fn load() -> Config {
        let path = env::var("HOME").unwrap() + "/.config/srhd/config.toml";
        let contents = fs::read_to_string(path).expect("Failed to read config file");
        let config = toml::from_str::<Config>(&contents).expect("Failed to parse config file");
        println!("Config loaded");
        return config;
    }

    fn run(&self, keys: &HeldKeys) {
        for binding in &self.binding {
            if keys.contains(&binding.key)
                && binding.mods.iter().all(|mod_key| keys.contains(mod_key))
            {
                binding.run();
            }
        }
    }
}

impl Binding {
    #[allow(dead_code)]
    fn run(&self) {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .spawn()
            .expect("Failed to run command");
    }
}

use rdev::{listen, Event};

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
