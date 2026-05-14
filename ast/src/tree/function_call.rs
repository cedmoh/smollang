use super::{Expression, Expressions};

/// Represents a function call.
///
/// # Examples
///
/// ```
/// print 'Hello, World!'
/// ```
///
/// ```
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
