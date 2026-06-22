use crate::Span;

/// An identifier, which is a name that can be used to refer to a value or a
/// function. Identifiers are used in variable declarations, function
/// declarations, and as the left-hand side of an assignment.
///
/// # Examples
///
/// ```smollang
/// print message
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    /// The name of the identifier
    pub id: String,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl Identifier {
    pub fn new(id: String, span: Span) -> Self {
        Self { id, span }
    }

    pub fn synthetic(id: String) -> Self {
        Self::new(id, Span::DUMMY)
    }
}
