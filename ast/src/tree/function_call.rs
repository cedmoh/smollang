use crate::{Expression, Expressions, Span};

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

    /// The location of the AST node in the source code
    pub span: Span,
}

impl FunctionCall {
    pub fn new(
        callee: Box<Expression>,
        arguments: FunctionCallArguments,
        span: Span,
    ) -> Self {
        Self {
            callee,
            arguments,
            span,
        }
    }

    pub fn synthetic(
        callee: impl Into<Expression>,
        arguments: FunctionCallArguments,
    ) -> Self {
        Self::new(Box::new(callee.into()), arguments, Span::DUMMY)
    }

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

    span: Option<Span>,
}

impl FunctionCallBuilder {
    pub fn new(callee: impl Into<Expression>) -> Self {
        Self {
            callee: Box::new(callee.into()),
            arguments: FunctionCallArguments::default(),
            span: None,
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

    pub fn span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn build(self) -> FunctionCall {
        FunctionCall::new(
            self.callee,
            self.arguments,
            self.span.unwrap_or(Span::DUMMY),
        )
    }
}
