use crate::{Expression, Expressions};

/// A pipe expression, which allows for chaining multiple expressions together.
///
/// # Example
///
/// ```
/// 1 |> add(it, 2) |> multiply(it, 3)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Pipe {
    pub expressions: Expressions,
}

impl Pipe {
    pub fn new(expressions: Expressions) -> Self {
        Self { expressions }
    }

    pub fn builder(
        first_expression: Expression,
        second_expression: Expression,
    ) -> PipeBuilder {
        PipeBuilder::new(first_expression, second_expression)
    }
}

pub struct PipeBuilder {
    expressions: Expressions,
}

impl PipeBuilder {
    pub fn new(
        first_expression: Expression,
        second_expression: Expression,
    ) -> Self {
        Self {
            expressions: Expressions::new(vec![
                first_expression,
                second_expression,
            ]),
        }
    }

    pub fn with_expression(mut self, expression: Expression) -> Self {
        self.expressions.add_expression(expression);
        self
    }

    pub fn add_expression(&mut self, expression: Expression) -> &mut Self {
        self.expressions.add_expression(expression);
        self
    }

    pub fn build(self) -> Pipe {
        Pipe::new(self.expressions)
    }
}
