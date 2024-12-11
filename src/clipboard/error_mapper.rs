pub(crate) fn map_arboard_err_to_label(err: arboard::Error) -> nu_protocol::LabeledError {
    nu_protocol::LabeledError::new(format!("Clipboard Error: {}", err.to_string()))
}
