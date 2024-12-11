use super::{arboard_provider::with_clipboard_instance, clipboard::Clipboard};
use nu_protocol::LabeledError;
pub struct ClipBoardWindows {}

impl ClipBoardWindows {
    pub fn new() -> Self {
        ClipBoardWindows {}
    }
}

impl Clipboard for ClipBoardWindows {}
