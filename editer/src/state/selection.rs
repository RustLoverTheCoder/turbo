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

/*
    A single selection range.
    When allowMultipleSelections is enabled, a selection may hold multiple ranges.
    By default, selections hold exactly one range.
*/
#[derive(Debug, PartialEq, Eq)]
pub struct SelectionRange {
    /*
       The lower boundary of the range.
    */
    from: usize,

    /*
       The upper boundary of the range.
    */
    to: usize,

    /*
       The anchor of the range—the side that doesn't move when you extend it.
    */
    anchor: usize,

    /*
       The head of the range, which is moved when the range is extended.
    */
    head: usize,

    /*
       True when anchor and head are at the same position.
    */
    empty: bool,

    /*
       If this is a cursor that is explicitly associated with the character on one of its sides, this returns the side.
        -1 means the character before its position,
        1 the character after,
        and 0 means no association.
    */
    assoc: Assoc,

    /*
       The bidirectional text level associated with this cursor, if any.
    */
    bidiLevel: Option<usize>,

    /*
       The goal column (stored vertical offset) associated with a cursor.
       This is used to preserve the vertical position when moving across lines of different length.
    */
    goalColumn: Option<usize>,
}

impl SelectionRange {
    // Map this range through a change, producing a valid range in the updated document.
    fn map(&self) -> SelectionRange {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Assoc {
    before = -1,
    after = 1,
    association = 0,
}
