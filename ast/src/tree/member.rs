use crate::{Expression, Span};

/// A member expression, which represents accessing a member of an object or a
/// module.     
///
/// # Examples
///
/// ```smollang
/// math.sin(3.14)
/// ```
/// In this example, `math.sin` is a member expression that accesses the `sin`
/// member of the `math` module.
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    /// The object or module being accessed.
    pub object: Box<Expression>,

    /// The member being accessed.
    pub property: Box<Expression>,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl Member {
    pub fn new(
        object: Box<Expression>,
        property: Box<Expression>,
        span: Span,
    ) -> Self {
        Self {
            object,
            property,
            span,
        }
    }

    /// Creates a synthetic member expression with a dummy span.
    pub fn synthetic(object: Expression, property: Expression) -> Self {
        Self::new(Box::new(object), Box::new(property), Span::DUMMY)
    }
}
