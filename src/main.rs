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
    // Start the daemon
    Start,
    // Stop the daemon,
    Stop,
}

fn main() {
    let args = Args::parse();

    if let Some(cmd) = args.cmd {
        let s = srhd::service::Service::new();
        match cmd {
            Commands::Start => s.start().expect("Failed to start daemon"),
            Commands::Stop => s.stop().expect("Failed to stop daemon"),
        }
    } else {
        srhd::config::srhd_process();
    }
}
