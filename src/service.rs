use std::{fs, io::Error};

pub fn install(ctl: &launchctl::Service) -> Result<(), Error> {
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
</plist>",
        ctl.name,
        // this right here is just ass
        std::env::current_exe().unwrap().to_str().unwrap()
    );

    Ok(fs::write(ctl.plist_path.clone(), plist)?)
}

// out of use currently
// pub fn uninstall(path: String) -> Result<(), Error> {
//     Ok(fs::remove_file(path)?)
// }
