use crate::Value;

/// The result of evaluating an expression or statement.
#[derive(Debug, PartialEq)]
pub enum EvaluationResult {
    /// A value that was produced by evaluating an expression.
    Value(Value),

    /// A value that was returned by a return statement.
    Return(Value),

    /// A value that was thrown by a throw statement.
    Throw(Value),
}

impl EvaluationResult {
    /// Returns `true` if the evaluation result is a throw, and `false` otherwise.
    pub fn is_err(&self) -> bool {
        matches!(self, EvaluationResult::Throw(_))
    }

    /// Returns `true` if the evaluation result is a value or return, and `false` otherwise.
    pub fn is_ok(&self) -> bool {
        matches!(
            self,
            EvaluationResult::Value(_) | EvaluationResult::Return(_)
        )
    }
}
