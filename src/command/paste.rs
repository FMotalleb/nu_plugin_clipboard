use crate::{
    clipboard::clipboard::{create_clipboard, Clipboard},
    utils::json::json_to_value,
    ClipboardPlugins,
};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, IntoPipelineData, LabeledError, PipelineData, Type, Value};

pub struct ClipboardPaste;

impl ClipboardPaste {
    pub fn new() -> Self {
        Self
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
        "Outputs the current value in clipboard"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let text = create_clipboard(engine.get_plugin_config().ok().flatten()).get_text()?;
        if text.trim().is_empty() {
            return Err(LabeledError::new("Empty clipboard".to_string()));
        }

        if call.has_flag("raw").unwrap_or(false) {
            return Ok(Value::string(text, call.head).into_pipeline_data());
        }

        match nu_json::from_str::<nu_json::Value>(&text) {
            Ok(value) => json_to_value(value, call.head).map(|v| v.into_pipeline_data()),
            Err(nu_json::Error::Syntax(_, _, _)) => {
                Ok(Value::string(text, call.head).into_pipeline_data())
            }
            Err(e) => Err(LabeledError::new(format!(
                "JSON Deserialization error: {}",
                e
            ))),
        }
    }
}
