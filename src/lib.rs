use std::{env, fs, io::Error, process::Command};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Service {
    launchctl_path: String,
    service_target: String,
    domain_target: String,
    uid: String,
    srhd_path: String,
    plist_path: String,
    user: String,
    error_log_path: String,
    out_log_path: String,
    name: String,
}

#[allow(dead_code)]
impl Service {
    pub fn new() -> Self {
        let user = env::var("USER").unwrap();
        let home = env::var("HOME").unwrap();
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
            user,
            name,
        }
    }

    pub fn stop(&self) -> Result<(), Error> {
        let mut print = Command::new(&self.launchctl_path);
        print.args(vec!["print", &self.service_target]);

        let mut kill = Command::new(&self.launchctl_path);
        kill.args(vec!["kill", "SIGTERM", &self.service_target]);

        let mut bootout = Command::new(&self.launchctl_path);
        bootout.args(vec!["bootout", &self.domain_target, &self.plist_path]);

        if !print.status()?.success() {
            kill.status()?;
        } else {
            bootout.status()?;
        }

        Ok(())
    }

    /// Attemps to start the service
    pub fn start(&self) -> Result<(), Error> {
        // print
        let mut print = Command::new(&self.launchctl_path);
        print.args(vec!["print", &self.service_target]);

        // bootstrap
        let mut bootstrap = Command::new(&self.launchctl_path);
        bootstrap.args(vec!["bootstrap", &self.domain_target, &self.plist_path]);

        // kickstart
        let mut kickstart = Command::new(&self.launchctl_path);
        kickstart.args(vec!["kickstart", &self.plist_path]);

        // enable
        let mut enable = Command::new(&self.launchctl_path);
        enable.args(vec!["enable", &self.service_target]);

        self.install()?;

        // This print message checks if the service is not bootstrapped
        if !print.status()?.success() {
            enable.status()?; // try to enable, if not already
            bootstrap.status()?; // bootstrap the service
        } else {
            kickstart.status()?; // restart the service
        }

        Ok(())
    }

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

    fn uninstall(&self) -> Result<(), Error> {
        Ok(fs::remove_file(&self.plist_path)?)
    }
}
