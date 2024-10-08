use std::{env, fs, io::Error, process::Command};

/// Wrapper for the launchctl system command to manage the service
#[derive(Debug)]
#[allow(dead_code)]
pub struct Service {
    /// Name of the service, constant (com.sylvanfranklin.srhd)
    name: String,
    /// usually /bin/launchctl
    service_target: String,
    /// The target of the domain (gui/<uid>)
    launchctl_path: String,
    /// The target of the service (gui/<uid>/com.sylvanfranklin.srhd)
    domain_target: String,
    /// uid, typically 501
    uid: String,
    /// Path to the srhd binary
    srhd_path: String,
    /// Path to the plist file typically ~/Library/LaunchAgents/com.sylvanfranklin.srhd.plist
    plist_path: String,
    /// Path to the error log file (/tmp/srhd_<user>.err.log)
    error_log_path: String,
    /// Path to the out log file (/tmp/srhd_<user>.out.log)
    out_log_path: String,
}

#[allow(dead_code)]
impl Service {
    pub fn new() -> Self {
        let user = match env::var("USER") {
            Ok(user) => user,
            Err(_) => panic!("$USER environment variable not found, abort."),
        };
        let home = match env::var("HOME") {
            Ok(home) => home,
            Err(_) => panic!("$HOME environment variable not found, abort."),
        };

        let name = "com.sylvanfranklin.srhd".to_string();
        let uid = "501".to_string();

        Service {
            launchctl_path: "/bin/launchctl".to_string(),
            srhd_path: format!(
                "{}/documents/projects/srhd/target/debug/internal_process",
                home
            ),
            plist_path: format!("{}/Library/LaunchAgents/{}.plist", home, name),
            error_log_path: format!("/tmp/srhd_{}.out.log", user),
            out_log_path: format!("/tmp/srhd_{}.out.log", user),
            service_target: format!("gui/{}/{}", uid, name),
            domain_target: format!("gui/{}", uid),
            uid,
            name,
        }
    }

    // todo make this into an external struct, maybe even a library
    fn launchctl_cmd(&self, args: Vec<&str>) -> Result<(), Error> {
        let mut command = Command::new(&self.launchctl_path);
        command
            .args(args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()?;
        Ok(())
    }

    fn is_bootstrapped(&self) -> bool {
        let mut command = Command::new(&self.launchctl_path);
        command
            .args(vec!["print", &self.service_target])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap_or_else(|_| panic!("Failed to execute command: {}", &self.launchctl_path))
            .success()
    }

    /// Attemps to stop the service
    pub fn stop(&self) -> Result<(), Error> {
        if !self.is_bootstrapped() {
            self.launchctl_cmd(vec!["kill", "SIGTERM", &self.service_target])?;
        } else {
            self.launchctl_cmd(vec!["bootout", &self.domain_target, &self.plist_path])?;
        }

        Ok(())
    }

    /// Attemps to start the service
    pub fn start(&self) -> Result<(), Error> {
        self.install()?;

        // This print message checks if the service is not bootstrapped
        if !self.is_bootstrapped() {
            self.launchctl_cmd(vec!["enable", &self.service_target])?;
            self.launchctl_cmd(vec!["bootstrap", &self.domain_target, &self.plist_path])?;
        } else {
            self.launchctl_cmd(vec!["kickstart", &self.plist_path])?;
        }

        Ok(())
    }

    /// Copies the plist file
    fn install(&self) -> Result<(), Error> {
        // TODO find a ugly way to format this
        let plist = format!(
"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple Computer//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
        <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
 	     <false/>
 	     <key>Crashed</key>
 	     <true/>
    </dict>
    <key>StandardOutPath</key>
    <string>/tmp/srhd_sylvanfranklin.out.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/srhd_sylvanfranklin.err.log</string>
    <key>ProcessType</key>
    <string>Interactive</string>
    <key>Nice</key>
    <integer>-20</integer>
</dict>
</plist>", self.name, self.srhd_path);

        Ok(fs::write(&self.plist_path, plist)?)
    }

    /// Removes the plist file
    fn uninstall(&self) -> Result<(), Error> {
        Ok(fs::remove_file(&self.plist_path)?)
    }
}
