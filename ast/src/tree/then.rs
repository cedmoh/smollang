use crate::Expression;

/// Represents a "then" expression, which is used in conditional statements.
///
/// # Example
///
/// ```
/// (x > 0) then 'positive' else 'non-positive'
/// ```
pub struct Then {
    /// The condition expression that is evaluated to determine which branch to execute.
    pub condition: Box<Expression>,

    /// The expression that is executed if the condition evaluates to true.
    pub then_body: Box<Expression>,

    /// The expression that is executed if the condition evaluates to false.
    /// This is optional, and if it is not provided, nothing will be executed when the condition is false.
    pub else_body: Option<Box<Expression>>,
}
