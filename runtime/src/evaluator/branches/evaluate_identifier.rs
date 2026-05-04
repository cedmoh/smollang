use ast::Identifier;

use crate::{Evaluator, Scope, ScopeItem, Value, evaluator::evaluation_result::EvaluationResult};

/// Function to evaluate an identifier expression.
pub fn evaluate_identifier(
    identifier: Identifier,
    _evaluator: &Evaluator,
    scope: &Scope,
) -> EvaluationResult {
    match scope.lookup(&identifier.id) {
        Some(ScopeItem::Variable { value, .. }) => EvaluationResult::Value(
            // TODO: Remove when value is copyable.
            value.clone(),
        ),
        Some(ScopeItem::Function) => todo!("Function identifiers are not yet supported."),
        None => EvaluationResult::Throw(Value::String(format!(
            "Variable '{}' is not defined.",
            identifier.id
        ))),
    }
}
