use crate::{PrettyPrint, write_field_label, write_node_label};

use super::*;
use std::fmt;

/// A match expression, which is a control flow construct that allows you to
/// match an expression against a series of patterns and execute the
/// corresponding block of code for the first pattern that matches.
///
/// # Example
///
/// ```
/// tuple match [x,y] do 'couple', _ do 'other'
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub expression: Box<Expression>,
    pub branches: Vec<MatchArm>,
}

/// A match arm, which consists of a pattern and a block of code to execute if
/// the pattern matches.
///
/// # Example
///
/// ```
/// 'couple' -> print 2
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    /// The pattern to match against the expression.
    pub pattern: Pattern,

    /// The block of code to execute if the pattern matches.
    pub body: Block,
}

impl PrettyPrint for Match {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Match")?;
        write_field_label(f, indent + 2, "expression")?;
        self.expression.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "branches")?;
        self.branches.fmt_with_indent(f, indent + 4)
    }
}

impl PrettyPrint for MatchArm {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "MatchArm")?;
        write_field_label(f, indent + 2, "pattern")?;
        self.pattern.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "body")?;
        self.body.fmt_with_indent(f, indent + 4)
    }
}
