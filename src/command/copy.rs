use crate::clipboard::clipboard::Clipboard;
use crate::ClipboardPlugins;
use crate::{clipboard::clipboard::create_clipboard, utils::json};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, IntoPipelineData, LabeledError, PipelineData, Signature, Type, Value};

pub struct ClipboardCopy;

impl ClipboardCopy {
    pub fn new() -> ClipboardCopy {
        ClipboardCopy {}
    }
    fn format_json(input: &Value) -> Result<String, LabeledError> {
        let json_value =
            json::value_to_json_value(&input).map(|v| nu_json::to_string_with_indent(&v, 4));

        match json_value {
            Ok(Ok(text)) => Ok(text.to_owned()), // Return the owned String
            Ok(Err(err)) => Err(LabeledError::new(format!(
                "JSON Serialization Error: {}",
                err,
            ))),
            Err(err) => Err(LabeledError::new(format!("JSON Conversion Error: {}", err))),
        }
    }
    fn copy(engine: &EngineInterface, input: &Value) -> Result<(), LabeledError> {
        let text: Result<String, LabeledError> = match input {
            Value::String { val, .. } => Ok(val.to_owned()),
            _ => Self::format_json(input),
        };

        match text.map(|text| {
            create_clipboard(engine.get_plugin_config().ok().unwrap_or(None))
                .copy_text(text.as_str())
        }) {
            Ok(Ok(_)) => Ok(()),
            Err(err) | Ok(Err(err)) => Err(err),
        }
    }
}

impl PluginCommand for ClipboardCopy {
    type Plugin = ClipboardPlugins;

    fn name(&self) -> &str {
        "clipboard copy"
    }

    fn signature(&self) -> Signature {
        Signature::build("clipboard copy")
            .input_output_types(vec![(Type::Any, Type::Any)])
            .category(Category::System)
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let value = input.into_value(call.head);
        match value {
            Ok(value) => {
                if let Err(err) = Self::copy(engine, &value) {
                    return Err(err);
                }
                Ok(value.into_pipeline_data())
            }
            Err(err) => Err(LabeledError::new(err.to_string())),
        }
    }

    fn description(&self) -> &str {
        "copy the input into the clipboard"
    }
}
