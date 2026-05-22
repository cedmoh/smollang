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
    pub fn add_expression(&mut self, expression: Expression) -> &mut Self {
        self.body.items.push(expression);
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
