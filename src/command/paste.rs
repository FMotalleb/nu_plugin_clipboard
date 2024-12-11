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
            .switch(
                "from-json",
                "formats input if its in json format",
                Some('j'),
            )
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
        let text = create_clipboard().get_text()?;
        if text.trim().is_empty() {
            return Err(LabeledError::new("Empty clipboard".to_string()));
        }
        if let Ok(true) = call.has_flag("from-json") {
            let value: Result<nu_json::Value, nu_json::Error> = nu_json::from_str(&text);
            return match value {
                Ok(value) => Ok(json_to_value(value, call.head)?.into_pipeline_data()),
                Err(err) => Err(LabeledError::new(format!(
                    "Json Deserializer exception: {}",
                    err.to_string()
                ))),
            };
            // formatter
        }
        Ok(Value::string(text, call.head).into_pipeline_data())
    }
}
