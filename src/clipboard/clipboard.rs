#[cfg(target_os = "linux")]
use std::io::Error;

use nu_protocol::LabeledError;

use super::arboard_provider::with_clipboard_instance;

pub enum CheckResult {
    Continue,
    #[cfg(target_os = "linux")]
    Exit(String, i32),
}
#[cfg(target_os = "linux")]
fn no_daemon(config: Option<nu_protocol::Value>) -> Result<bool, Error> {
    match config {
        None => Ok(false),
        Some(nu_protocol::Value::Record { val, .. }) => {
            return no_daemon(val.get("NO_DAEMON").cloned());
        }
        Some(nu_protocol::Value::Bool { val, .. }) => Ok(val),
        Some(nu_protocol::Value::String { val, .. }) => match val.as_str() {
            "true" | "True" | "1" => Ok(true),
            _ => Ok(false),
        },
        Some(nu_protocol::Value::Int { val, .. }) => Ok(val == 1),
        _ => Ok(true),
    }
}
#[cfg(target_os = "linux")]
pub fn create_clipboard(config: Option<nu_protocol::Value>) -> impl Clipboard {
    crate::clipboard::linux::ClipBoardLinux::new(!no_daemon(config).unwrap_or(false))
}
#[cfg(not(target_os = "linux"))]
pub fn create_clipboard(_: Option<nu_protocol::Value>) -> impl Clipboard {
    #[cfg(target_os = "linux")]
    {
        crate::clipboard::linux::ClipBoardLinux::new(!no_daemon(config).unwrap_or(false))
    }
    #[cfg(target_os = "macos")]
    {
        crate::clipboard::mac_os::ClipBoardMacos::new()
    }
    #[cfg(target_os = "windows")]
    {
        crate::clipboard::windows::ClipBoardWindows::new()
    }
}

pub trait Clipboard {
    fn pre_execute_check(&self) -> CheckResult {
        CheckResult::Continue
    }
    fn copy_text(&self, text: &str) -> Result<(), LabeledError> {
        with_clipboard_instance(|clip| clip.set_text(text))
    }
    fn get_text(&self) -> Result<String, LabeledError> {
        with_clipboard_instance(|clip| clip.get_text())
    }
}
