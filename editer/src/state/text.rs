#[derive(Debug, PartialEq, Eq)]
pub struct Text {
    /*
    The length of the string.
     */
    length: usize,
    /*
    The number of lines in the string (always >= 1).
     */
    lines: usize,
}