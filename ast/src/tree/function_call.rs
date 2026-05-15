use super::*;
use crate::{PrettyPrint, write_field_label, write_node_label};
use std::fmt;

/// Represents a function call.
///
/// # Examples
///
/// ```smollang
/// print 'Hello, World!'
/// ```
///
/// ```smollang
/// add(1, 2)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    /// The expression representing the function being called.
    pub callee: Box<Expression>,

    /// The arguments passed to the function call.
    pub arguments: FunctionCallArguments,
}

impl FunctionCall {
    pub fn builder(callee: Expression) -> FunctionCallBuilder {
        FunctionCallBuilder::new(callee)
    }
}

/// Represents the arguments passed to a function call.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FunctionCallArguments {
    pub expressions: Expressions,
}

pub struct FunctionCallBuilder {
    callee: Box<Expression>,

    arguments: FunctionCallArguments,
}

impl FunctionCallBuilder {
    pub fn new(callee: Expression) -> Self {
        Self {
            callee: Box::new(callee),
            arguments: FunctionCallArguments::default(),
        }
    }

    pub fn with_argument(mut self, argument: Expression) -> Self {
        self.arguments.expressions.add_expression(argument);
        self
    }

    pub fn add_argument(&mut self, argument: Expression) -> &mut Self {
        self.arguments.expressions.add_expression(argument);
        self
    }

    pub fn build(self) -> FunctionCall {
        FunctionCall {
            callee: self.callee,
            arguments: self.arguments,
        }
    }
}

impl PrettyPrint for FunctionCall {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "FunctionCall")?;
        write_field_label(f, indent, "callee")?;
        self.callee.fmt_with_indent(f, indent + 2)?;
        write_field_label(f, indent, "arguments")?;
        self.arguments.fmt_with_indent(f, indent + 2)
    }
}

impl PrettyPrint for FunctionCallArguments {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        self.expressions.fmt_with_indent(f, indent)
    }
}
