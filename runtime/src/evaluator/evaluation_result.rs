use crate::Value;

/// The result of evaluating an expression or statement.
#[derive(Debug)]
pub enum EvaluationResult {
    /// A value that was produced by evaluating an expression.
    Value(Value),

    /// A value that was returned by a return statement.
    Return(Value),

    /// A value that was thrown by a throw statement.
    Throw(Value),
}
