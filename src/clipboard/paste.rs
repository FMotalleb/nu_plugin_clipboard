use arboard::Clipboard;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, ErrorLabel, LabeledError, PipelineData, Type, Value};

use crate::ClipboardPlugins;

pub struct ClipboardPaste;

impl ClipboardPaste {
    pub fn new() -> ClipboardPaste {
        ClipboardPaste {}
    }
}
impl PluginCommand for ClipboardPaste {
    type Plugin = ClipboardPlugins;

    fn name(&self) -> &str {
        "clipboard paste"
    }

    fn signature(&self) -> nu_protocol::Signature {
        nu_protocol::Signature::build("clipboard paste")
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .category(Category::Experimental)
    }

    fn description(&self) -> &str {
        "outputs the current value in clipboard"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let mut clipboard = match Clipboard::new() {
            Ok(clip) => clip,
            Err(err) => {
                return Err(LabeledError {
                    msg: err.to_string(),
                    labels: vec![ErrorLabel {
                        text: "clipboard error".to_string(),
                        span: call.head,
                    }],
                    code: None,
                    url: None,
                    help: None,
                    inner: vec![],
                })
            }
        };
        match clipboard.get_text() {
            Ok(value) => Ok(PipelineData::Value(Value::string(value, call.head), None)),
            Err(err) => Err(LabeledError {
                msg: err.to_string(),
                labels: vec![ErrorLabel {
                    text: "get clipboard content error".to_string(),
                    span: call.head,
                }],
                code: None,
                url: None,
                help: None,
                inner: vec![],
            }),
        }
    }
}
