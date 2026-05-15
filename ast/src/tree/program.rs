use crate::{
    Expression, Expressions, PrettyPrint, write_field_label, write_node_label,
};
use std::fmt;

/// A program represents a File. It consists of a sequence of expressions that
/// will be executed in order.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program {
    /// The body of the program, which consists of a sequence of expressions
    /// that will be executed in order.
    pub body: Expressions,
}

impl Program {
    pub fn new(body: Expressions) -> Self {
        Self { body }
    }

    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ProgramBuilder {
    body: Expressions,
}

impl ProgramBuilder {
    pub fn add_expression(&mut self, expression: Expression) -> &mut Self {
        self.body.items.push(expression);
        self
    }

    pub fn build(self) -> Program {
        Program { body: self.body }
    }
}

impl PrettyPrint for Program {
    /// # Example
    /// ```
    /// Program
    /// body:
    ///   Expression > ...
    ///   Expression > ...
    ///   ...
    /// ```
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Program")?;
        write_field_label(f, indent, "body")?;
        self.body.fmt_with_indent(f, indent + 2)
    }
}
