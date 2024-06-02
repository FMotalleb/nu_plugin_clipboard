use crate::utils::json;
use crate::ClipboardPlugins;
use arboard::Clipboard;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, ErrorLabel, LabeledError, PipelineData, Signature, Span, Type, Value};
use std::{env, process};

pub struct ClipboardCopy;
const DAEMON_FLAG: &str = match cfg!(feature = "enforce-daemon") {
    true => "disable",
    false => "enable",
};
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
                if let Err(err) = clip.set().text(env::args().nth(2).as_deref().unwrap_or("")) {
                    println!("copy exception: {}", err);
                }
            }
            Err(err) => {
                println!("exception: {}", err);
            }
        }
    }

    #[cfg(target_os = "linux")]
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
        return match input {
            Value::String { val, .. } => Self::copy_str(val.to_owned(), as_daemon, span),
            _ => {
                let json_value = json::value_to_json_value(&input);
                return match json_value {
                    Ok(json_obj) => {
                        let json_str = nu_json::to_string_with_indent(&json_obj, 4);
                        match json_str {
                            Ok(json_str) => Self::copy_str(json_str, as_daemon, span),
                            Err(err) => Some(LabeledError {
                                msg: format!("cannot convert json object to string: {}", err),
                                labels: vec![ErrorLabel {
                                    text: "input to string conversion error".to_string(),
                                    span: input.span(),
                                }],
                                code: None,
                                url: None,
                                help: None,
                                inner: vec![],
                            }),
                        }
                    }
                    Err(err) => Some(LabeledError {
                        msg: format!("cannot convert input to json object: {}", err),
                        labels: vec![ErrorLabel {
                            text: "input to string conversion error".to_string(),
                            span: input.span(),
                        }],
                        code: None,
                        url: None,
                        help: None,
                        inner: vec![],
                    }),
                };
            }
        };
    }
    fn create_clipboard(span: Span) -> Result<Clipboard, LabeledError> {
        Clipboard::new().map_err(|err| LabeledError {
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

    fn _direct_copy(clip: &mut Clipboard, data: String, span: Span) -> Result<(), LabeledError> {
        clip.set_text(data).map_err(|err| LabeledError {
            msg: err.to_string(),
            labels: vec![ErrorLabel {
                text: "copy error".to_string(),
                span,
            }],
            code: None,
            url: None,
            help: None,
            inner: vec![],
        })
    }
    #[cfg(target_os = "windows")]
    fn copy_str(data: String, as_daemon: bool, span: Span) -> Option<LabeledError> {
        let mut clipboard = Self::create_clipboard(span).ok()?;
        Self::_direct_copy(&mut clipboard, data, span).ok()?;
        None
    }
    #[cfg(target_os = "linux")]
    fn copy_str(data: String, as_daemon: bool, span: Span) -> Option<LabeledError> {
        let mut clipboard = Self::create_clipboard(span).ok()?;

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
            Self::_direct_copy(&mut clipboard, data, span).ok()?;
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
                .input_output_types(
                    vec![
                        (Type::String, Type::String),
                        (Type::Record(Box::new([])), Type::String),
                        (Type::Table(Box::new([])), Type::String),
                        (Type::List(Box::new(Type::Any)), Type::String),
                    ]
                )
                .category(Category::Experimental)
        } else {
            Signature::build("clipboard copy")
                // .input_output_types(vec![(Type::String, Type::String)])
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
        let is_daemon = call.has_flag(&format!("{}-daemon", DAEMON_FLAG));
        match (is_daemon, value) {
            (Ok(is_daemon), Ok(value)) => {
                let copy_result =
                    Self::copy(value, cfg!(target_os = "linux") && is_daemon, call.head);
                if let Some(err) = copy_result {
                    return Err(err);
                }
                return Ok(PipelineData::Value(value.to_owned(), None));
            }
            _ => {}
        }
        Err(LabeledError::new("Failed to copy"))
    }
}
