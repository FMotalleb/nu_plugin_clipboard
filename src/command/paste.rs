use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, IntoPipelineData, LabeledError, PipelineData, Type, Value};

use crate::{
    clipboard::clipboard::{create_clipboard, Clipboard},
    utils::json::json_to_value,
    ClipboardPlugins,
};

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
            .switch("raw", "disable json formatting", Some('r'))
            .input_output_types(vec![(Type::Nothing, Type::Any)])
            .category(Category::System)
    }

    fn description(&self) -> &str {
        "outputs the current value in clipboard"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let text = create_clipboard(engine.get_plugin_config().ok().unwrap_or(None)).get_text()?;
        if text.trim().is_empty() {
            return Err(LabeledError::new("Empty clipboard".to_string()));
        }
        if let Ok(true) = call.has_flag("raw") {
            Ok(Value::string(text, call.head).into_pipeline_data())
        } else {
            let value: Result<nu_json::Value, nu_json::Error> = nu_json::from_str(&text);
            match value {
                Ok(value) => Ok(json_to_value(value, call.head)?.into_pipeline_data()),
                Err(nu_json::Error::Syntax(_, _, _)) => {
                    Ok(Value::string(text, call.head).into_pipeline_data())
                }
                Err(nu_json::Error::Io(err)) => Err(LabeledError::new(format!(
                    "Json Deserializer IO exception: {}",
                    err.to_string()
                ))),
                Err(nu_json::Error::FromUtf8(err)) => Err(LabeledError::new(format!(
                    "Json Deserializer FromUtf8 exception: {}",
                    err.to_string()
                ))),
            }
        }
    }
}
