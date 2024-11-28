use crate::listener::HeldKeys;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(strum::Display, Debug, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "lowercase")]
/// Custom macOS key names for the config file
pub enum Key {
    // letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    // modifiers
    Control,
    Shift,
    Option,
    Command,
    Fn,
    Backspace,
    CapsLock,
    Delete,
    DownArrow,
    End,
    Escape,
    // mods
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Home,
    LeftArrow,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    Space,
    Tab,
    UpArrow,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    // special
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    Comma,
    Dot,
    Slash,
    Insert,
    Unknown(u32),
    // keypad
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
}

impl Into<rdev::Key> for Key {
    fn into(self) -> rdev::Key {
        match self {
            Self::A => rdev::Key::KeyA,
            Self::B => rdev::Key::KeyB,
            Self::C => rdev::Key::KeyC,
            Self::D => rdev::Key::KeyD,
            Self::E => rdev::Key::KeyE,
            Self::F => rdev::Key::KeyF,
            Self::G => rdev::Key::KeyG,
            Self::H => rdev::Key::KeyH,
            Self::I => rdev::Key::KeyI,
            Self::J => rdev::Key::KeyJ,
            Self::K => rdev::Key::KeyK,
            Self::L => rdev::Key::KeyL,
            Self::M => rdev::Key::KeyM,
            Self::N => rdev::Key::KeyN,
            Self::O => rdev::Key::KeyO,
            Self::P => rdev::Key::KeyP,
            Self::Q => rdev::Key::KeyQ,
            Self::R => rdev::Key::KeyR,
            Self::S => rdev::Key::KeyS,
            Self::T => rdev::Key::KeyT,
            Self::U => rdev::Key::KeyU,
            Self::V => rdev::Key::KeyV,
            Self::W => rdev::Key::KeyW,
            Self::X => rdev::Key::KeyX,
            Self::Y => rdev::Key::KeyY,
            Self::Z => rdev::Key::KeyZ,
            Self::Control => rdev::Key::ControlLeft,
            Self::Shift => rdev::Key::ShiftLeft,
            Self::Option => rdev::Key::Alt,
            Self::Command => rdev::Key::MetaLeft,
            Self::Fn => rdev::Key::Function,
            Self::Backspace => rdev::Key::Backspace,
            Self::CapsLock => rdev::Key::CapsLock,
            Self::Delete => rdev::Key::Delete,
            Self::DownArrow => rdev::Key::DownArrow,
            Self::End => rdev::Key::End,
            Self::Escape => rdev::Key::Escape,
            Self::F1 => rdev::Key::F1,
            Self::F2 => rdev::Key::F2,
            Self::F3 => rdev::Key::F3,
            Self::F4 => rdev::Key::F4,
            Self::F5 => rdev::Key::F5,
            Self::F6 => rdev::Key::F6,
            Self::F7 => rdev::Key::F7,
            Self::F8 => rdev::Key::F8,
            Self::F9 => rdev::Key::F9,
            Self::F10 => rdev::Key::F10,
            Self::F11 => rdev::Key::F11,
            Self::F12 => rdev::Key::F12,
            Self::Home => rdev::Key::Home,
            Self::LeftArrow => rdev::Key::LeftArrow,
            Self::PageDown => rdev::Key::PageDown,
            Self::PageUp => rdev::Key::PageUp,
            Self::Return => rdev::Key::Return,
            Self::RightArrow => rdev::Key::RightArrow,
            Self::Space => rdev::Key::Space,
            Self::Tab => rdev::Key::Tab,
            Self::UpArrow => rdev::Key::UpArrow,
            Self::Pause => rdev::Key::Pause,
            Self::NumLock => rdev::Key::NumLock,
            Self::BackQuote => rdev::Key::BackQuote,
            Self::Num1 => rdev::Key::Num1,
            Self::Num2 => rdev::Key::Num2,
            Self::Num3 => rdev::Key::Num3,
            Self::Num4 => rdev::Key::Num4,
            Self::Num5 => rdev::Key::Num5,
            Self::Num6 => rdev::Key::Num6,
            Self::Num7 => rdev::Key::Num7,
            Self::Num8 => rdev::Key::Num8,
            Self::Num9 => rdev::Key::Num9,
            Self::Num0 => rdev::Key::Num0,
            Self::Minus => rdev::Key::Minus,
            Self::Equal => rdev::Key::Equal,
            Self::LeftBracket => rdev::Key::LeftBracket,
            Self::RightBracket => rdev::Key::RightBracket,
            Self::SemiColon => rdev::Key::SemiColon,
            Self::Quote => rdev::Key::Quote,
            Self::BackSlash => rdev::Key::BackSlash,
            Self::IntlBackslash => rdev::Key::IntlBackslash,
            Self::Comma => rdev::Key::Comma,
            Self::Dot => rdev::Key::Dot,
            Self::Slash => rdev::Key::Slash,
            Self::Insert => rdev::Key::Insert,
            Self::Unknown(u32) => rdev::Key::Unknown(u32),
            Self::KpReturn => rdev::Key::KpReturn,
            Self::KpMinus => rdev::Key::KpMinus,
            Self::KpPlus => rdev::Key::KpPlus,
            Self::KpMultiply => rdev::Key::KpMultiply,
            Self::KpDivide => rdev::Key::KpDivide,
            Self::Kp0 => rdev::Key::Kp0,
            Self::Kp1 => rdev::Key::Kp1,
            Self::Kp2 => rdev::Key::Kp2,
            Self::Kp3 => rdev::Key::Kp3,
            Self::Kp4 => rdev::Key::Kp4,
            Self::Kp5 => rdev::Key::Kp5,
            Self::Kp6 => rdev::Key::Kp6,
            Self::Kp7 => rdev::Key::Kp7,
            Self::Kp8 => rdev::Key::Kp8,
            Self::Kp9 => rdev::Key::Kp9,
            Self::KpDelete => rdev::Key::KpDelete,
        }
    }
}

/// Config Values that will be mapped to rdev::Key
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum Mods {
    Command,
    Control,
    Shift,
    Option,
    CapsLock,
    Fn,
}

/// Config Values for each binding
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Binding {
    key: Key,
    command: String,
    mods: Vec<Mods>,
}

impl Into<HeldKeys> for &Binding {
    fn into(self) -> HeldKeys {
        HeldKeys {
            command: self.mods.contains(&Mods::Command),
            control: self.mods.contains(&Mods::Control),
            shift: self.mods.contains(&Mods::Shift),
            option: self.mods.contains(&Mods::Option),
            function: self.mods.contains(&Mods::Fn),
            capslock: self.mods.contains(&Mods::CapsLock),
            key: Some(self.key),
        }
    }
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
        let content: Bindings =
            toml::from_str::<Bindings>(&raw_file_contents).expect("Failed to parse config file");

        println!("Config loaded successfully.");
        println!("{} Bindings active.", content.bindings.len());
        Config { path, content }
    }

    /// Attempts to execute all the commands
    pub fn execute_command(&self, held: &HeldKeys) -> bool {
        for binding in &self.content.bindings {
            if held.binding_pressed(binding) {
                binding.run();
                return true;
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
