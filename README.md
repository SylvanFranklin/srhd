# SRHD
**S**imple **R**ust **H**otkey **D**aemon, 

**SRHD** is a minimal and lightweight key binding service for MacOS similar to **skhd**. It can be run as a service using the native `launchctl` to interact with `launchd` via a plist file. It is configured through a simple toml file scheme. 

## Installation 
The first time **srhd** starts it will request access to input monitoring.
After being granted access you must restart the service for the change to take
effect. __Secure Keyboard Entry__ must be disabled in whatever terminal
emulator **srhd** is started from. (I did not find this to be a problem in alacritty)

**Homebrew**
```
brew install srhd
srhd start
```

**Cargo**
```
cargo install srhd
srhd start
```

**Source** 
```
git clone https://github.com/SylvanFranklin/srhd
cd srhd 
cargo run -- start
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
