use crate::{PrettyPrint, write_field_label, write_indent, write_node_label};

use super::*;
use std::fmt;

/// Represents a function declaration.
///
/// # Examples
///     
/// ```
/// add |x,y| x + y
/// ```
///
/// ```
/// hello |name| (
///     print('Hello, ' + name)
/// )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Option<Identifier>,
    pub params: FunctionParameters,
    pub body: Option<FunctionBody>,
}

/// Represents a function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub name: Identifier,
}

/// Represents the parameters of a function declaration.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionParameters {
    pub items: Vec<FunctionParameter>,
}

/// Represents the body of a function declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub body: Box<Expression>,
}

/// A return expression, which is used to return a value from a function.
///
/// # Example
///
/// ```
/// ret 5
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub expression: Option<Box<Expression>>,
}

impl PrettyPrint for FunctionDeclaration {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "FunctionDeclaration")?;

        write_field_label(f, indent + 2, "name")?;
        match &self.name {
            Some(name) => name.fmt_with_indent(f, indent + 4)?,
            None => {
                write_indent(f, indent + 4)?;
                writeln!(f, "null")?;
            }
        }

        write_field_label(f, indent + 2, "params")?;
        self.params.fmt_with_indent(f, indent + 4)?;

        write_field_label(f, indent + 2, "body")?;
        match &self.body {
            Some(body) => body.fmt_with_indent(f, indent + 4),
            None => {
                write_indent(f, indent + 4)?;
                writeln!(f, "null")
            }
        }
    }
}

impl PrettyPrint for FunctionParameter {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "FunctionParameter")?;
        write_field_label(f, indent + 2, "name")?;
        self.name.fmt_with_indent(f, indent + 4)
    }
}

impl PrettyPrint for FunctionParameters {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        for parameter in &self.items {
            parameter.fmt_with_indent(f, indent)?;
        }

        Ok(())
    }
}

impl PrettyPrint for FunctionBody {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        self.body.fmt_with_indent(f, indent)
    }
}

impl PrettyPrint for Return {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Return")?;
        write_field_label(f, indent + 2, "expression")?;

        match &self.expression {
            Some(expression) => expression.fmt_with_indent(f, indent + 4),
            None => {
                write_indent(f, indent + 4)?;
                writeln!(f, "null")
            }
        }
    }
}
