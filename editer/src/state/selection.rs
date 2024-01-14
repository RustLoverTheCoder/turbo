#[derive(Debug, PartialEq, Eq)]
pub enum EditorSelection {
    Text,
    Range(Range),
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    anchor: usize,
    head: Option<usize>,
}

impl EditorSelection {
    fn new_text() -> Self {
        EditorSelection::Text
    }

    fn new_range(anchor: usize, head: Option<usize>) -> Self {
        EditorSelection::Range(Range { anchor, head })
    }
}
