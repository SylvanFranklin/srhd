/// Serialize struct for the purpose of the config file  
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum Mods {
    Command,
    Control,
    Shift,
    Option,
}

/// Serialize struct for the purpose of the config file
#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Binding {
    key: rdev::Key,
    command: String,
    mods: Vec<Mods>,
}

/// Serialize struct for the purpose of the config file
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Bindings {
    bindings: Vec<Binding>,
}

#[allow(dead_code)]
pub struct Config {
    path: PathBuf,
    content: Bindings,
}

impl Config {
    /// always called internally, creates a new config file
    fn create_new_file(path: &PathBuf) -> Result<Vec<Binding>, std::io::Error> {
        let base_config: Vec<Binding> = vec![Binding {
            key: rdev::Key::KeyL,
            command: "echo 'Hello World'".to_string(),
            mods: vec![Mods::Shift, Mods::Control],
        }];

        let raw_toml_string =
            "[[bindings]]\nkey = \"KeyL\"\ncommand = \"echo 'hello world'\"\nmods = [\"Shift\", \"Control\"]";

        // create the directory
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, raw_toml_string)?;
        Ok(base_config)
    }

    /// Used for creating a new Config Object
    pub fn load() -> Config {
        // define the path right away, this can be used for the rest of the creation process, since
        // it's on the top level and will be handed down.
        let path: PathBuf =
            PathBuf::from(std::env::var("HOME").unwrap()).join(".config/srhd/srhd.toml");

        if !path.exists() {
            println!("Creating new config");
            Config::create_new_file(&path).unwrap();
        }

        let raw_file_contents: String = std::fs::read_to_string(&path).unwrap();
        let content = toml::from_str::<Bindings>(&raw_file_contents).unwrap();

        println!("Config loaded sucessfully.");
        println!("{} Bindings active.", content.bindings.len());
        Config { path, content }
    }

    /// Attempts to execute all the commands
    pub fn execute_command(&self, held: &HeldKeys) -> bool {
        for binding in &self.content.bindings {
            if let Some(key) = held.key {
                if key == binding.key
                    && binding.mods.iter().all(|modi| match modi {
                        Mods::Command => held.command,
                        Mods::Control => held.control,
                        Mods::Shift => held.shift,
                        Mods::Option => held.option,
                    })
                {
                    binding.run();
                    return true;
                }
            }
        }
        false
    }
}

impl Binding {
    pub fn run(&self) {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .spawn()
            .expect("Failed to run command");
    }
}

use crate::listener::HeldKeys;
use std::path::PathBuf;
