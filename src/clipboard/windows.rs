use super::{arboard_provider::with_clipboard_instance, clipboard::Clipboard};
use nu_protocol::LabeledError;
pub(crate) struct ClipBoardWindows;

impl ClipBoardWindows {
    pub fn new() -> Self {
        Self {}
    }
}

impl Clipboard for ClipBoardWindows {}
