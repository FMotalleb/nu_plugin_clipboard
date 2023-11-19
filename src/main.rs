use std::{env, ops::ControlFlow, process};

use arboard::{Clipboard, SetExtLinux};
use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::{Category, PluginSignature, Span, Type, Value};

pub struct Plugin;
const DAEMON_FLAG: &str = match cfg!(feature = "enforce-daemon") {
    true => "disable",
    false => "enable",
};
impl nu_plugin::Plugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        let mut sig = vec![];
        if cfg!(target_os = "linux") {
            sig.push(
                PluginSignature::build("clipboard copy")
                    .usage("copy the input into the clipboard")
                    .switch(
                        format!("{}-daemon",DAEMON_FLAG),
                        format!(
                            "cause copy action to {} the daemon feature (open a process in background), this fixes some errors in some Desktop environments if you are OK without it don't use it",
                            DAEMON_FLAG
                        ),
                        Some('d'),
                    )
                    .input_output_types(vec![(Type::String, Type::String)])
                    .category(Category::Experimental),
            )
        } else {
            sig.push(
                PluginSignature::build("clipboard copy")
                    .usage("copy the input into the clipboard")
                    .input_output_types(vec![(Type::String, Type::String)])
                    .category(Category::Experimental),
            )
        }
        sig.push(
            PluginSignature::build("clipboard paste")
                .usage("outputs the current value in clipboard")
                .input_output_types(vec![(Type::Nothing, Type::String)])
                .category(Category::Experimental),
        );

        return sig;
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "clipboard copy" => {
                if let Some(err) = copy(
                    input,
                    cfg!(target_os = "linux") && call.has_flag(&format!("{}-daemon", DAEMON_FLAG)),
                ) {
                    return Err(err);
                }
                return Ok(input.to_owned());
            }
            "clipboard paste" => {
                return paste(call.head);
            }
            _ => {
                let message = std::format!("{} is an unknown command", name).to_string();
                return Err(LabeledError {
                    label: "Unknown command".to_string(),
                    msg: message,
                    span: Some(call.head),
                });
            }
        };
    }
}

#[cfg(target_os = "linux")]
const DAEMONIZE_ARG: &str = "9020bba4f13c910db6211b87cb667614";

fn main() {
    if let ControlFlow::Break(_) = daemon_entry() {
        return;
    }
    nu_plugin::serve_plugin(&mut Plugin {}, nu_plugin::MsgPackSerializer {})
}

fn daemon_entry() -> ControlFlow<()> {
    #[cfg(target_os = "linux")]
    if env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG) {
        match Clipboard::new() {
            Ok(mut clip) => {
                if let Err(err) = clip
                    .set()
                    .wait()
                    .text(env::args().nth(2).as_deref().unwrap_or(""))
                {
                    println!("copy exception: {}", err);
                }
            }
            Err(err) => {
                println!("exception: {}", err);
            }
        }
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

fn copy(input: &Value, as_daemon: bool) -> Option<LabeledError> {
    let data: String = match input.as_string() {
        Ok(text) => text,
        Err(err) => {
            return Some(LabeledError {
                label: "input to string conversion error".to_string(),
                msg: err.to_string(),
                span: None,
            })
        }
    };
    let mut clipboard = match Clipboard::new() {
        Ok(clip) => clip,
        Err(err) => {
            return Some(LabeledError {
                label: "clipboard error".to_string(),
                msg: err.to_string(),
                span: None,
            })
        }
    };

    if cfg!(feature = "enforce-daemon") ^ as_daemon {
        match start_daemon(&data) {
            Ok(_) => {}
            Err(err) => {
                return Some(LabeledError {
                    label: "daemon initialization exception".to_string(),
                    msg: err.to_string(),
                    span: None,
                });
            }
        }
    } else {
        if let Err(err) = clipboard.set_text(data) {
            return Some(LabeledError {
                label: "copy error".to_string(),
                msg: err.to_string(),
                span: None,
            });
        }
    }
    None
}
fn paste(span: Span) -> Result<Value, LabeledError> {
    let mut clipboard = match Clipboard::new() {
        Ok(clip) => clip,
        Err(err) => {
            return Err(LabeledError {
                label: "clipboard error".to_string(),
                msg: err.to_string(),
                span: None,
            })
        }
    };
    match clipboard.get_text() {
        Ok(value) => Ok(Value::string(value, span)),
        Err(err) => Err(LabeledError {
            label: "get clipboard content error".to_string(),
            msg: err.to_string(),
            span: Some(span),
        }),
    }
}

fn start_daemon(data: &String) -> Result<process::Child, std::io::Error> {
    match env::current_exe() {
        Ok(exe) => {
            return process::Command::new(exe)
                .arg(DAEMONIZE_ARG)
                .arg(data)
                .stdin(process::Stdio::null())
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .current_dir("/")
                .spawn();
        }
        Err(err) => {
            return Err(err);
        }
    }
}
