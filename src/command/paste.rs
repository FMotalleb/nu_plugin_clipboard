use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, IntoPipelineData, LabeledError, PipelineData, Type, Value};

use crate::{
    clipboard::clipboard::{create_clipboard, Clipboard},
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
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .category(Category::Experimental)
    }

    fn description(&self) -> &str {
        "outputs the current value in clipboard"
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let text = create_clipboard(engine.get_plugin_config().ok().unwrap_or(None)).get_text()?;
        if text.trim().is_empty() {
            return Err(LabeledError::new("Empty clipboard".to_string()));
        }
        Ok(Value::string(text, call.head).into_pipeline_data())
    }
}
