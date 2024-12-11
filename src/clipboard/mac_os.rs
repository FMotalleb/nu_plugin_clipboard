use nu_protocol::LabeledError;

use super::{arboard_provider::with_clipboard_instance, clipboard::Clipboard};

pub(crate) struct ClipBoardMacos;

impl ClipBoardMacos {
    pub fn new() -> Self {
        Self {}
    }
}

impl Clipboard for ClipBoardMacos {}
