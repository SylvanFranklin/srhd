# SRHD
**S**imple **R**ust **H**otkey **D**aemon is a minimal and lightweight key
binding service for MacOS similar to **skhd**. It can be run as in the
background using the native `launchctl` to interact with `launchd` via a plist
file. This functionality has been offloaded to my [launchctl](https://github.com/sylvanfranklin/launchctl) Rust library. 

> [!WARNING]
> **WHY??** We have skhd, hammerspoon, aerospace, why use srhd? I don't claim
> this idea as original, nor do I claim that this project is in any way better
> than the others. The one up is that you can install it with cargo and it
> feels Rusty with the toml file, but that's about it. 

## Installation 
The first time **srhd** starts it will request access to input monitoring.
After being granted access you must restart the service for the change to take
effect. __Secure Keyboard Entry__ must be disabled in whatever terminal
emulator **srhd** is started from. In some cases if you update **srhd** or the
terminal emulator that is is started from, you may have to reset the
permissions in system settings. You may also have to give your terminal emulator full disk access for some reason, I swear I'm not scamming you though, look through the code yourself.


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
Requires Cargo and Rust.    
```sh
git clone https://github.com/SylvanFranklin/srhd
cd srhd 
cargo build --release 
target/release/srhd
```

## Usage
```
srhd [COMMAND]

Commands:
  start    Start launchctl login service via launchctl
  stop     Stop launchctl login service via launchctl
  restart  Restart launchctl login service via launchctl
  debug    Prints out info about the keybindings as they are pressed
  config   Prints path to the config file. Run `help config` for config options
  help     Print this message or the help of the given subcommand(s)
```

## Configuration

For now config is stored at, other options to be added in the future.
- `$HOME/.config/srhd/srhd.toml`

An empty config file will be created automatically the first time **srhd** is
started.

**Example Config:** 
```toml
# srhd.toml
[[bindings]]
key = "a"
command = "open /Applications/Firefox.app" # or any arbitrary shell script
mods = ["command", "option", "shift"]

[[bindings]]
key = "equals"
command = "echo 'hello from srhd'"
mods = ["control", "fn", "capslock"]
```
**Keys**
Below are some common keys that you may want to use in your config. The best
way to find out the code for a key is to run `srhd debug` and press the key in
question.

```
minus,
equal,
leftbracket,
rightbracket,
semicolon,
quote,
backslash,
comma,
dot,
slash,
insert
```

> [!WARNING]  
> **SRHD** is still in active development, and is lacking certain features like
> hot config reloading and comprehensive error messages. I'm working on rolling
> those out ASAP. There is also currently a bug where if permission are removed
> while srhd is running as a service the keyboard and mouse will become
> unresponsive, and a restart is required. 

## Debugging
stdout and stderr can be found at `/tmp/$USER_srhd.out.log` and
`/tmp/$USER_srhd.err.log` respectively. Running `srhd debug` can help resolve
issues with keys not matching up in the way that you expect. Most issues with
srhd not working are related to permissions and may sometimes require removing
and re-adding srhd from accessibility permissions.

## Contribution
Contribution is greatly appreciated feel free!
