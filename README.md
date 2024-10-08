# SRHD
**S**imple **R**ust **H**otkey **D**aemon, 

**SRHD** is a minimal and lightweight key binding service similar to **skhd**.
For the service / daemon it uses the MacOS native bin `launchctl` to interact with `launchd` via a plist file, and is therefore only usable on MacOS. It is configured through a simple toml scheme. 

## Installation 

The first time **srhd** starts it will request access to input monitoring. After being granted access you must restart the service for the change to take effect. 

__Secure Keyboard Entry__ must be disabled in whatever terminal emulator **srhd** is started from. 

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
Commands:
  start  
  stop   
  restart
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Configuration
The default configuration file is scanned for in the following locations in order:
- `$XDG_CONFIG_HOME/srhd/srhd.toml`
- `$HOME/.config/srhd/srhd.toml`

A config file will be created automatically the first time **srhd** is started.

Key bindings are defined as an array of toml objects. 

```toml
#srhd.toml
[[binding]]
key = "KeyA"
command = "echo 'any shell command'" # stdout goes to the out log 
mods = ["Meta", "Alt"]

[[binding.open]]
key = "KeyL"
command = "firefox"
mods = ["Control", "Shift"]
```
By default the command feild of a binding will execute shell script. Out of the
box **srhd** offers several alternate binding modes for convenience. *Note binding modes are simply snippets of shell, and can be easily defined by hand.

| mode | explanation |
|:--------------------------:|:----:|
| `[[binding]]` | execuses normal shell script |
| `[[binding.open]]` | opens / switches focus to application in $HOME/applications |
| `[[binding.quit]]` | gracefully quits application in $HOME/applications |
| `[[binding.kill]]` | force quits application in $HOME/applications |
