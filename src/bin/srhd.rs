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
        let s = srhd::Service::new();
        match cmd {
            Commands::Start => s.start().expect("Failed to start daemon"),
            Commands::Stop => s.stop().expect("Failed to stop daemon"),
        }
    } else {
        let home = std::env::var("HOME").unwrap();
        let mut proc = std::process::Command::new(
            home + "/documents/projects/srhd/target/debug/internal_process",
        );

        proc.status().expect("Failed to start internal process");
    }
}
