#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    /// A dummy span that can be used when the actual span is not available.
    pub const DUMMY: Span = Span {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    };

    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::DUMMY
    }
}
