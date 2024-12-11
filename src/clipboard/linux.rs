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
pub(crate) struct ClipBoardLinux {
    pub(crate) use_daemon: bool,
}

#[cfg(target_os = "linux")]
impl ClipBoardLinux {
    pub fn new(use_daemon: bool) -> Self {
        Self { use_daemon }
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
            Err(err) | Ok(Err(err)) => Err(nu_protocol::LabeledError::new(format!(
                "Failed to spawn daemon process: {}",
                err.to_string()
            ))),
        }
    }
    fn copy_with_daemon() -> Result<(), nu_protocol::LabeledError> {
        with_clipboard_instance(|clip: &mut arboard::Clipboard| {
            clip.clear()?;
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
        if self.use_daemon {
            self.request_daemon(text)
        } else {
            with_clipboard_instance(|clip: &mut arboard::Clipboard| clip.set_text(text))
        }
    }
}
