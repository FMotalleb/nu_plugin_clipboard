use std::{
    env,
    io::{stderr, stdout, Read, Write},
    process::{Command, Stdio},
};

use super::{
    arboard_provider::with_clipboard_instance,
    clipboard::{CheckResult, Clipboard},
};

use arboard::Error;
use nu_protocol::LabeledError;

const DAEMONIZE_ARG: &str = "daemon-copy";

pub(crate) struct ClipBoardLinux {
    pub(crate) use_daemon: bool,
}

impl ClipBoardLinux {
    pub fn new(use_daemon: bool) -> Self {
        Self { use_daemon }
    }
    fn is_daemon_request() -> bool {
        env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG)
    }
    fn request_daemon(&self, text: &str) -> Result<(), nu_protocol::LabeledError> {
        match env::current_exe().map(|exe| spawn_daemon(text, exe)) {
            Ok(res) => res,
            Err(err) => Err(nu_protocol::LabeledError::new(format!(
                "Failed to spawn daemon process: {}",
                err.to_string()
            ))),
        }
    }
    fn copy_with_daemon() -> Result<(), LabeledError> {
        with_clipboard_instance(|clip: &mut arboard::Clipboard| {
            clip.clear()?;
            let mut input = String::new();
            if let Err(err) = std::io::stdin().read_to_string(&mut input) {
                return Err(Error::Unknown {
                    description: format!("Failed to read from stdin: {}", err.to_string()),
                });
            }
            clip.set_text(input)
        })
    }
}

fn spawn_daemon(text: &str, exe: std::path::PathBuf) -> Result<(), LabeledError> {
    let child = Command::new(exe)
        .arg(DAEMONIZE_ARG)
        .stdin(Stdio::piped())
        .stdout(stdout())
        .stderr(stderr())
        .current_dir(env::temp_dir())
        .spawn();
    if let Err(err) = child {
        return Err(LabeledError::new(format!(
            "failed to spawn daemon process: {}",
            err
        )));
    }
    let mut child = child.unwrap();
    if let Some(mut stdin) = child.stdin.take() {
        if let Err(err) = stdin.write_all(text.as_bytes()) {
            return Err(LabeledError::new(format!(
                "Failed to write to stdin: {}",
                err
            )));
        }
    }

    // Optionally, wait for the child process to finish
    let status = child.wait();
    status
        .map(|_| ())
        .map_err(|e| LabeledError::new(e.to_string()))
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
