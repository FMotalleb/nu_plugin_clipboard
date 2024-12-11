use std::{
    env,
    io::{stderr, stdout},
    process::{Command, Stdio},
};

use super::{
    arboard_provider::with_clipboard_instance,
    clipboard::{CheckResult, Clipboard},
};

use arboard::SetExtLinux;
use nu_protocol::LabeledError;

const DAEMONIZE_ARG: &str = "daemon-copy";
pub(crate) struct ClipBoardLinux {}

#[cfg(target_os = "linux")]
impl ClipBoardLinux {
    pub fn new() -> Self {
        Self {}
    }
    fn is_daemon_request() -> bool {
        env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG)
    }
    fn request_daemon(&self, text: &str) -> Result<(), nu_protocol::LabeledError> {
        match env::current_exe().map(|exe| {
            Command::new(exe)
                .arg(DAEMONIZE_ARG)
                .arg(text)
                .stdin(Stdio::null())
                .stdout(stdout())
                .stderr(stderr())
                .current_dir(env::temp_dir())
                .spawn()
        }) {
            Ok(Ok(_)) => Ok(()),
            Err(err) | Ok(Err(err)) => Err(nu_protocol::LabeledError::new(err.to_string())),
        }
    }
    fn copy_with_daemon() -> Result<(), nu_protocol::LabeledError> {
        with_clipboard_instance(|clip: &mut arboard::Clipboard| {
            let _ = clip.clear();
            let args: Vec<String> = env::args().skip(2).collect();
            let data = args.join(" ");
            clip.set().wait().text(data)
        })
    }
}

impl Clipboard for ClipBoardLinux {
    fn pre_execute_check(&self) -> CheckResult {
        match Self::is_daemon_request() {
            true => match Self::copy_with_daemon() {
                Err(e) => CheckResult::Exit(e.msg, 1),
                _ => CheckResult::Exit("".to_string(), 0),
            },
            false => CheckResult::Continue,
        }
    }

    fn copy_text(&self, text: &str) -> Result<(), LabeledError> {
        self.request_daemon(text)
    }
}
