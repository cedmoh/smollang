use crate::{PrettyPrint, write_node_label, write_scalar_field};
use std::fmt;

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
    pub id: String,
}

impl Identifier {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

impl PrettyPrint for Identifier {
    /// # Examples
    ///
    /// ```text
    /// Identifier
    /// id: 'message'
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Identifier")?;
        write_scalar_field(f, indent, "id", format!("'{}'", self.id))
    }
}
