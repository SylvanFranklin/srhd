/// seperates keys and mods in order to make the serializer more straightforward.
/// there is some redundancy with these match statements, I would like to find a more elegant way
/// to do it in the future
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum Mods {
    Command,
    Control,
    Shift,
    Option,
}

/// Binding
#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Binding {
    key: rdev::Key,
    command: String,
    mods: Vec<Mods>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Bindings {
    shell: Vec<Binding>,
}

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

        Config { path, content }
    }

    /// Attempts to execute all the commands
    pub fn execute_commands(&self, held: &HeldKeys) {
        self.content
            .bindings
            .iter()
            .filter(|b| held.keys.contains(&b.key) && b.mods.iter().all(|e| held.mods.contains(e)))
            .for_each(|b| b.run());
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
