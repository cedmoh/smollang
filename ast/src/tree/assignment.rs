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
    /// The identifier being assigned to  
    /// TODO: This should allow for patterns and member access
    pub identifier: Identifier,

    /// The value being assigned
    pub value: Box<Expression>,
}

impl Assignment {
    pub fn new(identifier: Identifier, value: Box<Expression>) -> Self {
        Self { identifier, value }
    }
}
