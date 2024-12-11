use nu_protocol::LabeledError;

use super::arboard_provider::with_clipboard_instance;

pub enum CheckResult {
    Continue,
    Exit(String, i32),
}
pub fn create_clipboard() -> impl Clipboard {
    #[cfg(target_os = "linux")]
    {
        crate::clipboard::linux::ClipBoardLinux::new()
    }
    #[cfg(target_os = "macos")]
    {
        crate::clipboard::linux::ClipboardMacOs::new()
    }
    #[cfg(target_os = "windows")]
    {
        crate::clipboard::linux::ClipboardWindows::new()
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
