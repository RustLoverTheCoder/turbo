use super::extension::Extension;
use super::selection::EditorSelection;

// Options passed when creating an editor state.
pub struct EditorStateConfig {
    /*
    The initial document. Defaults to an empty document.
    Can be provided either as a plain string (which will be split into lines according to the value of the lineSeparator facet),
    or an instance of the Text class (which is what the state will use to represent the document).
    */
    doc: Option<String>,

    /*
    The starting selection. Defaults to a cursor at the very start of the document.
    */
    selection: EditorSelection,

    /*
    Extension(s) to associate with this state.
    */
    extensions: Extension,
}
