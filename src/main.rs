mod clipboard;

pub mod utils;
use nu_plugin::{PluginCommand};
use crate::clipboard::copy::ClipboardCopy;
use crate::clipboard::paste::ClipboardPaste;

pub struct ClipboardPlugins;

impl nu_plugin::Plugin for ClipboardPlugins {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin=Self>>> {
        vec![
            Box::new(ClipboardCopy::new()),
            Box::new(ClipboardPaste::new()),
        ]
    }
}


fn main() {
    if ClipboardCopy::is_daemon() {
        ClipboardCopy::daemon_entry();
        return;
    }
    nu_plugin::serve_plugin(&mut ClipboardPlugins {}, nu_plugin::MsgPackSerializer {})
}
