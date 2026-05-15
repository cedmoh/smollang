use crate::{PrettyPrint, write_field_label, write_node_label};

use super::*;

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
}

impl Assignment {
    pub fn new(left: Identifier, right: Box<Expression>) -> Self {
        Self { left, right }
    }
}

impl PrettyPrint for Assignment {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        write_node_label(f, indent, "Assignment")?;
        write_field_label(f, indent, "left")?;
        self.left.fmt_with_indent(f, indent + 2)?;
        write_field_label(f, indent, "right")?;
        self.right.fmt_with_indent(f, indent + 2)
    }
}
