use crate::{Expression, Span};

/// A break expression, which is used to exit a loop.
///
/// # Examples
///
/// ```smollang
/// break
/// ```
///
/// ```smollang
/// break 5
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Break {
    /// The optional expression to return when breaking out of a loop.
    pub expression: Option<Box<Expression>>,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl Break {
    pub fn new(expression: Option<Box<Expression>>, span: Span) -> Self {
        Self { expression, span }
    }

    /// Creates a synthetic break expression with a dummy span.
    pub fn synthetic(expression: Option<Expression>) -> Self {
        Self::new(expression.map(Box::new), Span::DUMMY)
    }
}
