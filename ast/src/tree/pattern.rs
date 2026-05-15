use std::fmt;

use crate::{PrettyPrint, write_node_label, write_scalar_field};

/// A pattern, which is used in match expressions to specify the structure of
/// the value being matched.
///
/// # Examples
///
/// ```text
/// [x, y]
/// ```
/// In this example, `[x, y]` is a pattern that matches a tuple with two
/// elements, where the first element is bound to `x` and the second element is
/// bound to `y`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    /// The content of the pattern, which can be a literal, an identifier, a
    /// tuple, a list, or a wildcard.
    pub content: String,
}

impl PrettyPrint for Pattern {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Pattern")?;
        write_scalar_field(
            f,
            indent + 2,
            "content",
            format!("'{}'", self.content),
        )
    }
}
