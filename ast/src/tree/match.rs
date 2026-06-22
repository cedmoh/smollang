use crate::{Expression, Pattern, Span};

/// A match expression, which is a control flow construct that allows you to
/// match an expression against a series of patterns and execute the
/// corresponding block of code for the first pattern that matches.
///
/// # Examples
///
/// ```smollang
/// tuple match [x,y] do 'couple', _ do 'other'
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    /// The expression being matched against the patterns.
    pub expression: Box<Expression>,

    /// The branches of the match expression, which consist of patterns and
    /// the corresponding blocks of code to execute if the patterns match.
    pub branches: Vec<MatchArm>,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl Match {
    pub fn new(
        expression: Box<Expression>,
        branches: Vec<MatchArm>,
        span: Span,
    ) -> Self {
        Self {
            expression,
            branches,
            span,
        }
    }

    /// Creates a synthetic match expression with a dummy span.
    pub fn synthetic(expression: Expression, branches: Vec<MatchArm>) -> Self {
        Self::new(Box::new(expression), branches, Span::DUMMY)
    }

    pub fn builder(expression: Expression) -> MatchBuilder {
        MatchBuilder::new(expression)
    }
}

#[derive(Debug, Clone)]
pub struct MatchBuilder {
    expression: Expression,
    branches: Vec<MatchArm>,
    span: Option<Span>,
}

impl MatchBuilder {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            branches: Vec::new(),
            span: None,
        }
    }

    pub fn with_expression(mut self, expression: Expression) -> Self {
        self.expression = expression;
        self
    }

    pub fn with_branch(mut self, branch: MatchArm) -> Self {
        self.branches.push(branch);
        self
    }

    pub fn add_branch(&mut self, branch: MatchArm) -> &mut Self {
        self.branches.push(branch);
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

    pub fn build(self) -> Match {
        Match::new(
            Box::new(self.expression),
            self.branches,
            self.span.unwrap_or(Span::DUMMY),
        )
    }
}

/// A match arm, which consists of a pattern and a block of code to execute if
/// the pattern matches.
///
/// # Examples
///
/// ```smollang
/// 'couple' -> print 2
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    /// The pattern to match against the expression.
    pub pattern: Pattern,

    /// The block of code to execute if the pattern matches.
    pub body: Expression,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl MatchArm {
    pub fn new(pattern: Pattern, body: Expression, span: Span) -> Self {
        Self {
            pattern,
            body,
            span,
        }
    }

    /// Creates a synthetic match arm with a dummy span.
    pub fn synthetic(pattern: Pattern, body: Expression) -> Self {
        Self::new(pattern, body, Span::DUMMY)
    }
}
