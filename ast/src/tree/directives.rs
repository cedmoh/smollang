use crate::{Span, Use};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Directives {
    /// The directives in the program.
    pub items: Vec<Directive>,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl Directives {
    pub fn new(items: Vec<Directive>, span: Span) -> Self {
        Self { items, span }
    }

    /// Creates a synthetic directives node with a dummy span.
    pub fn synthetic(items: Vec<Directive>) -> Self {
        Self::new(items, Span::DUMMY)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Directive {
    Use(Use),
}
