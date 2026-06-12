use crate::{Directive, Directives, Expression, Expressions};

/// A program represents a File. It consists of a sequence of expressions that
/// will be executed in order.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Program {
    pub directives: Directives,
    /// The body of the program, which consists of a sequence of expressions
    /// that will be executed in order.
    pub body: Expressions,
}

impl Program {
    pub fn new(directives: Directives, body: Expressions) -> Self {
        Self { directives, body }
    }

    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ProgramBuilder {
    directives: Directives,
    body: Expressions,
}

impl ProgramBuilder {
    /// Adds an expression to the program body, returning a mutable reference to
    /// the builder for chaining.
    pub fn add_expression(&mut self, expression: Expression) -> &mut Self {
        self.body.items.push(expression);
        self
    }

    /// Adds an expression to the program body, returning a new builder with the
    /// expression added.
    pub fn with_expression(
        mut self,
        expression: impl Into<Expression>,
    ) -> Self {
        self.add_expression(expression.into());
        self
    }

    /// Sets the expressions of the program body, replacing any existing
    /// expressions.
    pub fn expressions(mut self, expressions: Vec<Expression>) -> Self {
        self.body.items = expressions;
        self
    }

    pub fn add_directive(&mut self, directive: Directive) -> &mut Self {
        self.directives.items.push(directive);
        self
    }

    pub fn build(self) -> Program {
        Program {
            directives: self.directives,
            body: self.body,
        }
    }
}
