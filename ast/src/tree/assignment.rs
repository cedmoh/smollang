use super::*;

/// An assignment expression
///
/// # Example
///
/// ```
/// x = 5
/// ```
/// In this example, `x = 5` is an assignment expression that assigns the value `5` to the variable `x`.
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub identifier: Identifier,
    pub value: Box<Expression>,
}
