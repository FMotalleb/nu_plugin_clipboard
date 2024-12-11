use nu_protocol::LabeledError;

use super::{arboard_provider::with_clipboard_instance, clipboard::Clipboard};

pub struct ClipBoardMacos {}

impl ClipBoardMacos {
    pub fn new() -> Self {
        ClipBoardMacos {}
    }
}

impl Clipboard for ClipBoardMacos {}
