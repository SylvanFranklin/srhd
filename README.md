S(imple) R(ust) H(otkey) D(aemon), is a minimal and lightweight key
binding service inspired by **skhd**. Like **skhd** it uses `launchctl`, and is therefore only usable on MacOS.

### Install 
The first time **srhd** starts it will request access to input monitoring.
Restart the service for the change to take effect.  

Secure Keyboard Entry must be disabled in whatever terminal **srhd** is run
from.

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

### Usage
```
Commands:
  start  
  stop   
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Config
The default configuration file is located at 

- `$XDG_CONFIG_HOME/srhd/srhd.toml`
- `$HOME/.config/srhd/srhd.toml`

A config file will be created automatically the first time **srhd** is started.

Key bindings are defined as toml objects. Use `binding.sh` to execute any arbitrary shell command, or `binding.open` as a shorthand for opening an application in `$HOME/Applications`. More shorthand notation is likely in the future, as they improve ergonomics without breaking any configs.  

```toml
[[binding.open]]
key = "KeyL"
command = "firefox"
mods = ["Control", "Shift"]

[[binding.sh]]
key = "KeyA"
command = "echo 'any shell command'" # stdout goes to the out log 
mods = ["Meta", "Alt"]
```

