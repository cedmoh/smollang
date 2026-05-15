use crate::{PrettyPrint, write_field_label, write_node_label};

use super::*;
use std::fmt;

/// A member expression, which represents accessing a member of an object or a
/// module.     
/// # Example
///
/// ```
/// math.sin(3.14)
/// ```
/// In this example, `math.sin` is a member expression that accesses the `sin`
/// member of the `math` module.
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    ///
    pub chain: Expressions,
}

impl PrettyPrint for Member {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Member")?;
        write_field_label(f, indent + 2, "chain")?;
        self.chain.fmt_with_indent(f, indent + 4)
    }
}
