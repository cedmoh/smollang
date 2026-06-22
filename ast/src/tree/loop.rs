use crate::{Expression, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    /// The body of the loop.
    pub body: Box<Expression>,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl Loop {
    pub fn new(body: Box<Expression>, span: Span) -> Self {
        Self { body, span }
    }

    /// Creates a synthetic loop expression with a dummy span.
    pub fn synthetic(body: impl Into<Expression>) -> Self {
        Self::new(Box::new(body.into()), Span::DUMMY)
    }
}
