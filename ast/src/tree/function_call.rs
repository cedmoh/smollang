use super::*;

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
    pub fn new(callee: impl Into<Expression>) -> Self {
        Self {
            callee: Box::new(callee.into()),
            arguments: FunctionCallArguments::default(),
        }
    }

    pub fn with_argument(mut self, argument: impl Into<Expression>) -> Self {
        self.arguments.expressions.add_expression(argument.into());
        self
    }

    pub fn add_argument(
        &mut self,
        argument: impl Into<Expression>,
    ) -> &mut Self {
        self.arguments.expressions.add_expression(argument.into());
        self
    }

    pub fn build(self) -> FunctionCall {
        FunctionCall {
            callee: self.callee,
            arguments: self.arguments,
        }
    }
}
