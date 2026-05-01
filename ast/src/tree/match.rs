use super::{Block, Expression, Pattern};

/// A match expression, which is a control flow construct that allows you to match an expression against
/// a series of patterns and execute the corresponding block of code for the first pattern that matches.
///
/// # Example
///
/// ```
/// tuple match [x,y] do 'couple', _ do 'other'
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub expression: Box<Expression>,
    pub branches: Vec<MatchArm>,
}

/// A match arm, which consists of a pattern and a block of code to execute if the pattern matches.
///
/// # Example
///
/// ```
/// 'couple' -> print 2
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    /// The pattern to match against the expression.
    pub pattern: Pattern,

    /// The block of code to execute if the pattern matches.
    pub body: Block,
}
