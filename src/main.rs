mod clipboard;
mod command;
pub mod utils;
use std::{
    io::{self, stderr, stdout, Write},
    process::exit,
};

use crate::command::copy::ClipboardCopy;
use crate::command::paste::ClipboardPaste;
use clipboard::clipboard::{create_clipboard, CheckResult, Clipboard};
use nu_plugin::PluginCommand;

pub struct ClipboardPlugins;

impl nu_plugin::Plugin for ClipboardPlugins {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(ClipboardCopy::new()),
            Box::new(ClipboardPaste::new()),
        ]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}

fn main() -> Result<(), io::Error> {
    match create_clipboard(None).pre_execute_check() {
        CheckResult::Continue => Ok(nu_plugin::serve_plugin(
            &mut ClipboardPlugins {},
            nu_plugin::MsgPackSerializer {},
        )),
        CheckResult::Exit(message, code) => {
            if code != 0 {
                writeln!(stderr(), "Error ({}): {}", code, message)?;
            } else if !message.is_empty() {
                writeln!(stdout(), "{}", message)?;
            }
            exit(code)
        }
    }
}
