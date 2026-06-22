use crate::{Expression, Span};

/// A return expression, which is used to return a value from a function.
///
/// # Examples
///
/// ```smollang
/// ret 5
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    /// The optional expression to return from the function.
    pub expression: Option<Box<Expression>>,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl Return {
    pub fn new(expression: Option<Box<Expression>>, span: Span) -> Self {
        Self { expression, span }
    }

    /// Creates a synthetic return expression with a dummy span.
    pub fn synthetic(expression: Option<Expression>) -> Self {
        Self::new(expression.map(Box::new), Span::DUMMY)
    }
}
