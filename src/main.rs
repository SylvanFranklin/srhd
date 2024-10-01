use clap::Parser;
use srhd::srhd_process;

/// Simple Rust Hotkey Daemon
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Parser, Debug)]
enum Commands {
    // Start the daemon
    Start,
    // Stop the daemon,
    Stop,
}

fn main() {
    let args = Args::parse();

    if let Some(cmd) = args.cmd {
        match cmd {
            Commands::Start => println!("Starting daemon"),
            Commands::Stop => println!("Stopping daemon"),
        }
    } else {
        srhd_process();
    }
}
