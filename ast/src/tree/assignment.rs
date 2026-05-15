use crate::{PrettyPrint, write_field_label, write_node_label};

use super::*;

/// An assignment expression
///
/// # Example
///
/// ```
/// x = 5
/// ```
/// In this example, `x = 5` is an assignment expression that assigns the value
/// `5` to the variable `x`.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    /// The identifier being assigned to  
    /// TODO: This should allow for patterns and member access
    pub identifier: Identifier,

    /// The value being assigned
    pub value: Box<Expression>,
}

impl Assignment {
    pub fn new(identifier: Identifier, value: Box<Expression>) -> Self {
        Self { identifier, value }
    }
}

impl PrettyPrint for Assignment {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        write_node_label(f, indent, "Assignment")?;
        write_field_label(f, indent + 2, "identifier")?;
        self.identifier.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "value")?;
        self.value.fmt_with_indent(f, indent + 4)
    }
}
