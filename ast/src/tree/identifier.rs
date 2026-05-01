use std::fmt::Display;

/// An identifier, which is a name that can be used to refer to a value or a function.
/// Identifiers are used in variable declarations, function declarations, and as the left-hand side of an assignment.
///
/// # Example
///
/// ```
/// print message
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub id: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T: Into<String>> From<T> for Identifier {
    fn from(id: T) -> Self {
        Identifier { id: id.into() }
    }
}
