use crate::{ClipboardPlugins, DAEMON_FLAG};
use arboard::{Clipboard, SetExtLinux};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, ErrorLabel, LabeledError, PipelineData, Signature, Span, Type, Value};
use std::{env, process};
pub struct ClipboardCopy;

const DAEMONIZE_ARG: &str = "9020bba4f13c910db6211b87cb667614";
impl ClipboardCopy {
    pub fn new() -> ClipboardCopy {
        ClipboardCopy {}
    }
    pub fn is_daemon() -> bool {
        env::args().nth(1).as_deref() == Some(DAEMONIZE_ARG)
    }
    pub fn daemon_entry() {
        #[cfg(target_os = "linux")]
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
    }

    pub fn start_daemon(data: &String) -> Result<process::Child, std::io::Error> {
        return match env::current_exe() {
            Ok(exe) => process::Command::new(exe)
                .arg(DAEMONIZE_ARG)
                .arg(data)
                .stdin(process::Stdio::null())
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .current_dir("/")
                .spawn(),
            Err(err) => Err(err),
        };
    }

    fn copy(input: &Value, as_daemon: bool, span: Span) -> Option<LabeledError> {
        match input {
            Value::String { val, .. } => {
                return Self::copy_str(val.to_owned(), as_daemon, span);
            }
            // Value::Record {val,..}=> {
            //
            //     return Self::copy_str(val.to_owned(),as_daemon,span);
            // }
            _ => {
                return Some(LabeledError {
                    msg: "cannot convert input to string".to_string(),
                    labels: vec![ErrorLabel {
                        text: "input to string conversion error".to_string(),
                        span: input.span(),
                    }],
                    code: None,
                    url: None,
                    help: None,
                    inner: vec![],
                });
            }
        }
    }

    fn copy_str(data: String, as_daemon: bool, span: Span) -> Option<LabeledError> {
        let mut clipboard = match Clipboard::new() {
            Ok(clip) => clip,
            Err(err) => {
                return Some(LabeledError {
                    msg: err.to_string(),
                    labels: vec![ErrorLabel {
                        text: "clipboard error".to_string(),
                        span,
                    }],
                    code: None,
                    url: None,
                    help: None,
                    inner: vec![],
                })
            }
        };

        if (cfg!(feature = "enforce-daemon")) ^ as_daemon {
            match Self::start_daemon(&data) {
                Ok(_) => {}
                Err(err) => {
                    return Some(LabeledError {
                        msg: err.to_string(),
                        labels: vec![ErrorLabel {
                            text: "daemon initialization exception".to_string(),
                            span,
                        }],
                        code: None,
                        url: None,
                        help: None,
                        inner: vec![],
                    });
                }
            }
        } else {
            if let Err(err) = clipboard.set_text(data) {
                return Some(LabeledError {
                    msg: err.to_string(),
                    labels: vec![ErrorLabel {
                        text: "copy error".to_string(),
                        span,
                    }],
                    code: None,
                    url: None,
                    help: None,
                    inner: vec![],
                });
            }
        }
        None
    }
}

impl PluginCommand for ClipboardCopy {
    type Plugin = ClipboardPlugins;

    fn name(&self) -> &str {
        "clipboard copy"
    }

    fn signature(&self) -> Signature {
        if cfg!(target_os = "linux") {
            Signature::build("clipboard copy")
                .switch(
                    format!("{}-daemon",DAEMON_FLAG),
                    format!(
                        "cause copy action to {} the daemon feature (open a process in background), this fixes some errors in some Desktop environments if you are OK without it don't use it",
                        DAEMON_FLAG
                    ),
                    Some('d'),
                )
                .input_output_types(vec![(Type::String, Type::String)])
                .category(Category::Experimental)
        } else {
            Signature::build("clipboard copy")
                .input_output_types(vec![(Type::String, Type::String)])
                .category(Category::Experimental)
        }
    }

    fn usage(&self) -> &str {
        "copy the input into the clipboard"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let value = &input.into_value(call.head);
        if let Ok(use_daemon) = call.has_flag(&format!("{}-daemon", DAEMON_FLAG)) {
            let copy_result = Self::copy(value, cfg!(target_os = "linux") && use_daemon, call.head);
            if let Some(err) = copy_result {
                return Err(err);
            }
        }
        return Ok(PipelineData::Value(value.to_owned(), None));
    }
}
