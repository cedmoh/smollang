use crate::{
    Expression, PrettyPrint, write_field_label, write_indent, write_node_label,
};
use std::fmt;

/// Represents a "then" expression, which is used in conditional statements.
///
/// # Examples
///
/// Then expression with an else body:
///
/// ```smollang
/// (x > 0) then 'positive' else 'non-positive'
/// ```
///
/// Then expression without an else body:
///
/// ```smollang
/// (x > 0) then 'positive'
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Then {
    /// The condition expression that is evaluated to determine which branch to
    /// execute.
    pub condition: Box<Expression>,

    /// The expression that is executed if the condition evaluates to true.
    pub then_body: Box<Expression>,

    /// The expression that is executed if the condition evaluates to false.
    /// This is optional, and if it is not provided, nothing will be executed
    /// when the condition is false.
    pub else_body: Option<Box<Expression>>,
}

impl Then {
    /// Creates a new `Then` expression with the given condition, then body, and
    /// optional else body.
    pub fn new(
        condition: Expression,
        then_body: Expression,
        else_body: Option<Expression>,
    ) -> Self {
        Self {
            condition: Box::new(condition),
            then_body: Box::new(then_body),
            else_body: else_body.map(Box::new),
        }
    }

    pub fn builder(
        condition: Expression,
        then_body: Expression,
    ) -> ThenBuilder {
        ThenBuilder::new(condition, then_body)
    }
}

pub struct ThenBuilder {
    condition: Expression,
    then_body: Expression,
    else_body: Option<Expression>,
}

impl ThenBuilder {
    pub fn new(condition: Expression, then_body: Expression) -> Self {
        Self {
            condition,
            then_body,
            else_body: None,
        }
    }

    pub fn add_else_body(&mut self, else_body: Expression) -> &mut Self {
        self.else_body = Some(else_body);
        self
    }

    pub fn with_else_body(mut self, else_body: Expression) -> Self {
        self.else_body = Some(else_body);
        self
    }

    pub fn build(self) -> Then {
        Then::new(self.condition, self.then_body, self.else_body)
    }
}

impl PrettyPrint for Then {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Then")?;
        write_field_label(f, indent + 2, "condition")?;
        self.condition.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "then_body")?;
        self.then_body.fmt_with_indent(f, indent + 4)?;
        write_field_label(f, indent + 2, "else_body")?;

        match &self.else_body {
            Some(else_body) => else_body.fmt_with_indent(f, indent + 4),
            None => {
                write_indent(f, indent + 4)?;
                writeln!(f, "null")
            }
        }
    }
}
