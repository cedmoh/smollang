use crate::Span;

/// A continue expression, which is used to skip the current iteration of a
/// loop.
///
/// # Examples
///
/// ```smollang
/// cont
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Continue {
    /// The location of the AST node in the source code
    pub span: Span,
}

impl Continue {
    pub fn new(span: Span) -> Self {
        Self { span }
    }

    /// Creates a synthetic continue expression with a dummy span.
    pub fn synthetic() -> Self {
        Self::new(Span::DUMMY)
    }
}
