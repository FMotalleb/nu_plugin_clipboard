use super::error_mapper::map_arboard_err_to_label;
use nu_protocol::LabeledError;
pub(crate) fn with_clipboard_instance<
    U,
    F: FnOnce(&mut arboard::Clipboard) -> Result<U, arboard::Error>,
>(
    op: F,
) -> Result<U, LabeledError> {
    let mut clipboard = arboard::Clipboard::new().map_err(map_arboard_err_to_label)?;

    op(&mut clipboard).map_err(map_arboard_err_to_label)
}
