use super::*;

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
    pub expression: Box<Expression>,
    pub branches: Vec<MatchArm>,
}

impl Match {
    pub fn new(expression: Expression, branches: Vec<MatchArm>) -> Self {
        Self {
            expression: Box::new(expression),
            branches,
        }
    }

    pub fn builder(expression: Expression) -> MatchBuilder {
        MatchBuilder::new(expression)
    }
}

#[derive(Debug, Clone)]
pub struct MatchBuilder {
    expression: Expression,
    branches: Vec<MatchArm>,
}

impl MatchBuilder {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression,
            branches: Vec::new(),
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

    pub fn build(self) -> Match {
        Match::new(self.expression, self.branches)
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
}

impl MatchArm {
    pub fn new(pattern: Pattern, body: Expression) -> Self {
        Self { pattern, body }
    }
}
