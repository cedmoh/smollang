use crate::Value;

/// The result of evaluating an expression or statement.
#[derive(Debug)]
pub enum EvaluationResult {
    Value(Value),
    Return(Value),
    Throw(Value),
}
