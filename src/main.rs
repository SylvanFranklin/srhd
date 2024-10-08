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
    /// Restart the service
    Restart,
    /// Prints the path to the toml config file
    #[command(long_about = "keybindings...")]
    Config,
}

fn main() {
    let args = Args::parse();

    if let Some(cmd) = args.cmd {
        let s = srhd::service::Service::new();
        match cmd {
            Commands::Start => s.start().expect("Failed to start daemon"),
            Commands::Stop => s.stop().expect("Failed to stop daemon"),
            Commands::Restart => s.restart().expect("Failed to restart daemon"),
            Commands::Config => println!("{}", "todo"),
        }
    } else {
        srhd::config::srhd_process();
    }
}
