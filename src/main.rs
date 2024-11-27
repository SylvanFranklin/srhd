use std::path::PathBuf;

use clap::Parser;

/// Simple Rust Hotkey Daemon
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Start launchctl login service
    Start,
    /// Stop launchctl login service
    Stop,
    /// Restart launchctl login service
    Restart,
    /// Prints out info about the keybindings as they are pressed 
    Debug,
    /// Prints path to the config file. Run `help config` for config options.
    #[command(long_about = "")]
    Config,
}

fn main() {
    let args = Args::parse();

    // When started as a daemon, this will pass right through to the else
    // block, since there are no arguments passed via the plist file
    if let Some(cmd) = args.cmd {
        let service = launchctl::Service::new(
            "com.sylvanfranklin.srhd",
            PathBuf::from(std::env::current_exe().unwrap()),
        );
        srhd::service::install(&service).unwrap();

        use Commands::*;
        match cmd {
            Start => service.start().expect("Failed to start service"),
            Stop => service.stop().expect("Failed to stop service"),
            Restart => service.restart().expect("Failed to restart service"),
            Config => println!(".config/srhd/srhd.toml"),
            Debug => srhd::listener::srhd_process(true),
        }
    } else {
        srhd::listener::srhd_process(false);
    }
}
