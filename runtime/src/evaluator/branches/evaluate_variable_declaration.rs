use ast::VariableDeclaration;

use crate::{EvaluationResult, Evaluator, Scope, Value};

/// Evaluate a variable declaration by evaluating the initial value (if it exists) and adding the variable to the scope.
/// Returns `Value::Nil` if the variable declaration was successful, or an error if it was not.
pub fn evaluate_variable_declaration(
    variable_declaration: VariableDeclaration,
    evaluator: &Evaluator,
    scope: &mut Scope,
) -> EvaluationResult {
    let initial_value: Option<Value> = match variable_declaration.initial_value {
        Some(initial) => match evaluator.evaluate_expression(*initial, scope) {
            EvaluationResult::Value(value) => Some(value),
            EvaluationResult::Throw(err) => return EvaluationResult::Throw(err),
            EvaluationResult::Return(_) => {
                unreachable!(
                    "A return statement is not valid in the initial value of a variable declaration."
                )
            }
        },
        None => None,
    };

    // TODO: Add utility function for adding a variable to the scope and handling the result,
    // since this will be a common operation.
    match scope
        .add_variable(
            &variable_declaration.name.id,
            initial_value.unwrap_or(Value::Nil),
            variable_declaration.is_mutable,
        )
        .map_err(|error| Value::String(format!("Error declaring variable: {:?}", error)))
        .map(|_| Value::Nil)
    {
        Ok(value) => EvaluationResult::Value(value),
        Err(err) => EvaluationResult::Throw(err),
    }
}
