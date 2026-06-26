use crate::{Expression, Span};

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

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl Then {
    pub fn new(
        condition: Box<Expression>,
        then_body: Box<Expression>,
        else_body: Option<Box<Expression>>,
        span: Span,
    ) -> Self {
        Self {
            condition,
            then_body,
            else_body,
            span,
        }
    }

    /// Creates a new `Then` expression with the given condition, then body, and
    /// optional else body.
    pub fn synthetic(
        condition: impl Into<Expression>,
        then_body: impl Into<Expression>,
        else_body: Option<impl Into<Expression>>,
    ) -> Self {
        Self::new(
            Box::new(condition.into()),
            Box::new(then_body.into()),
            else_body.map(|e| Box::new(e.into())),
            Span::DUMMY,
        )
    }

    pub fn builder(
        condition: impl Into<Expression>,
        then_body: impl Into<Expression>,
    ) -> ThenBuilder {
        ThenBuilder::new(condition.into(), then_body.into())
    }
}

pub struct ThenBuilder {
    condition: Expression,
    then_body: Expression,
    else_body: Option<Expression>,
    span: Option<Span>,
}

impl ThenBuilder {
    pub fn new(
        condition: impl Into<Expression>,
        then_body: impl Into<Expression>,
    ) -> Self {
        Self {
            condition: condition.into(),
            then_body: then_body.into(),
            else_body: None,
            span: None,
        }
    }

    pub fn add_else_body(
        &mut self,
        else_body: impl Into<Expression>,
    ) -> &mut Self {
        self.else_body = Some(else_body.into());
        self
    }

    pub fn with_else_body(mut self, else_body: impl Into<Expression>) -> Self {
        self.else_body = Some(else_body.into());
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    pub fn build(self) -> Then {
        Then::new(
            Box::new(self.condition),
            Box::new(self.then_body),
            self.else_body.map(Box::new),
            self.span.unwrap_or(Span::DUMMY),
        )
    }
}
