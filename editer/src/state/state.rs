use super::{text::Text, selection::EditorSelection};

#[derive(Debug, PartialEq, Eq)]
pub struct EditorState {
    doc: Text,
    selection: EditorSelection,
}