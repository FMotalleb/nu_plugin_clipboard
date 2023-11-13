use arboard::Clipboard;
use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_protocol::{Category, PluginSignature, Span, Type, Value};

pub struct Plugin;

impl nu_plugin::Plugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("clipboard copy")
                .usage("copy the input into the clipboard")
                .input_output_types(vec![(Type::String, Type::String)])
                .category(Category::Experimental),
            PluginSignature::build("clipboard paste")
                .usage("outputs the current value in clipboard")
                .input_output_types(vec![(Type::Nothing, Type::String)])
                .category(Category::Experimental),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "clipboard copy" => {
                copy(input);
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
fn main() {
    nu_plugin::serve_plugin(&mut Plugin {}, nu_plugin::MsgPackSerializer {})
}

fn copy(input: &Value) -> Option<LabeledError> {
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

    if let Err(err) = clipboard.set_text(data) {
        return Some(LabeledError {
            label: "copy error".to_string(),
            msg: err.to_string(),
            span: None,
        });
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
