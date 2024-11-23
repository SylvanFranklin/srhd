# SRHD
**S**imple **R**ust **H**otkey **D**aemon is a minimal and lightweight key
binding service for MacOS similar to **skhd**. It can be run as in the
background using the native `launchctl` to interact with `launchd` via a plist
file. This functionality has been offloaded to my [launchctl](https://github.com/sylvanfranklin/launchctl) Rust library. 

> [!WARNING]  
> **SRHD** is still in active development, and is lacking certain features like
> hot config reloading and comprehensive error messages. I'm working on rolling
> those out ASAP. There is also currently a bug where if permission are removed
> while srhd is running as a service the keyboard and mouse will become
> unresponsive, and a restart is required. 

## Installation 
The first time **srhd** starts it will request access to input monitoring.
After being granted access you must restart the service for the change to take
effect. __Secure Keyboard Entry__ must be disabled in whatever terminal
emulator **srhd** is started from. In alacritty this process is quite hacky,
and for some reason requires removing input monitoring in system settings. When
something stops working I have found that it can mostly be resolved by toggling
access. 

**Homebrew**
```sh
brew tap sylvanfranklin/srhd 
brew install srhd
srhd start
```

**Cargo**
Requires cargo and rust.    
```sh
cargo install srhd
srhd start
```

**Source** 
Requires cargo and rust.    
```sh
git clone https://github.com/SylvanFranklin/srhd
cd srhd 
cargo run 
```

## Usage
```
Usage: srhd [COMMAND]

Commands:
  start    Start launchctl login service
  stop     Stop launchctl login service
  restart  Restart the service
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Configuration

For now config is stored at, other options to be added in the future.
- `$HOME/.config/srhd/srhd.toml`

An empty config file will be created automatically the first time **srhd** is started.

**Example Config:** 
```toml
# srhd.toml
[[binding]]
key = "KeyA"
command = "open /Applications/Firefox.app" # or any arbitrary shell script
mods = ["Meta", "Alt"]
```

## Debugging
stdout and stderr can be found at `/tmp/$USER_srhd.out.log` and
`/tmp/$USER_srhd.err.log` respecively.

## Contribution
Contribution is greatly appreciated feel free!

