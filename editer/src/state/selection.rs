pub enum EditorSelection {
    Text,
    Range(Range),
}

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
