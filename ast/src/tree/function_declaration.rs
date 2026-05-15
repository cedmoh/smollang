use crate::{
    PrettyPrint, write_empty, write_field_label, write_node_label, write_none,
};

use super::*;
use std::fmt;

/// Represents a function declaration.
///
/// # Examples
///
/// Empty function declaration:
///
/// ```
/// empty ||
/// ```
///
/// Function declaration with parameters and body:
///     
/// ```
/// add |x,y| x + y
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Option<Identifier>,
    pub params: FunctionParameters,
    pub body: Option<FunctionBody>,
}

impl FunctionDeclaration {
    pub fn new(
        name: Option<Identifier>,
        params: FunctionParameters,
        body: Option<FunctionBody>,
    ) -> Self {
        Self { name, params, body }
    }

    pub fn builder() -> FunctionDeclarationBuilder {
        FunctionDeclarationBuilder::new()
    }
}

/// Represents a function parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    // TODO: Add support for patterns as function parameters
    pub name: Identifier,
}

impl FunctionParameter {
    pub fn new(name: Identifier) -> Self {
        Self { name }
    }
}

/// Represents the parameters of a function declaration.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionParameters {
    pub items: Vec<FunctionParameter>,
}

impl FunctionParameters {
    pub fn new(items: Vec<FunctionParameter>) -> Self {
        Self { items }
    }
}

/// Represents the body of a function declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub body: Option<Box<Expression>>,
}

impl FunctionBody {
    pub fn new(body: Expression) -> Self {
        Self {
            body: Some(Box::new(body)),
        }
    }
}

pub struct FunctionDeclarationBuilder {
    name: Option<Identifier>,
    params: FunctionParameters,
    body: Option<FunctionBody>,
}

impl FunctionDeclarationBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            params: FunctionParameters::default(),
            body: None,
        }
    }

    pub fn with_name(mut self, name: Identifier) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_params(mut self, params: Vec<FunctionParameter>) -> Self {
        self.params = FunctionParameters::new(params);
        self
    }

    pub fn with_body(mut self, body: Expression) -> Self {
        self.body = Some(FunctionBody::new(body));
        self
    }

    pub fn name(&mut self, name: Identifier) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn body(&mut self, body: Expression) -> &mut Self {
        self.body = Some(FunctionBody::new(body));
        self
    }

    pub fn add_param(&mut self, param: FunctionParameter) -> &mut Self {
        self.params.items.push(param);
        self
    }

    pub fn build(self) -> FunctionDeclaration {
        FunctionDeclaration::new(self.name, self.params, self.body)
    }
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

        write_field_label(f, indent, "name")?;
        match &self.name {
            Some(name) => name.fmt_with_indent(f, indent + 2)?,
            None => {
                write_none(f, indent + 2)?;
            }
        }

        write_field_label(f, indent, "params")?;
        self.params.fmt_with_indent(f, indent + 2)?;

        write_field_label(f, indent, "body")?;
        match &self.body {
            Some(body) => body.fmt_with_indent(f, indent + 2),
            None => write_none(f, indent + 2),
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
        write_field_label(f, indent, "name")?;
        self.name.fmt_with_indent(f, indent + 2)
    }
}

impl PrettyPrint for FunctionParameters {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        if self.items.is_empty() {
            return write_empty(f, indent);
        }

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
        match &self.body {
            Some(body) => body.fmt_with_indent(f, indent),
            None => write_none(f, indent),
        }
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
            None => write_none(f, indent),
        }
    }
}
