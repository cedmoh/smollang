use crate::{Expression, Identifier, Span};

/// An assignment expression
///
/// # Examples
///
/// ```smollang
/// x = 5
/// ```
/// In this example, `x = 5` is an assignment expression that assigns the value
/// `5` to the variable `x`.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    /// The identifier being assigned to  
    /// TODO: This should allow for patterns and member access
    pub left: Identifier,

    /// The value being assigned
    pub right: Box<Expression>,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl Assignment {
    pub fn new(
        left: Identifier,
        right: impl Into<Expression>,
        span: Span,
    ) -> Self {
        Self {
            left,
            right: Box::new(right.into()),
            span,
        }
    }

    /// Creates a synthetic assignment expression with a dummy span.
    pub fn synthetic(left: Identifier, right: impl Into<Expression>) -> Self {
        Self::new(left, right, Span::DUMMY)
    }
}
