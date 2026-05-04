use crate::{EvaluationResult, Evaluator, Scope, Value};

pub fn evaluate_assignment(
    assignment: ast::Assignment,
    evaluator: &Evaluator,
    scope: &mut Scope,
) -> EvaluationResult {
    // Evaluate the expression on the right-hand side
    let value = match evaluator.evaluate_expression(*assignment.value, scope) {
        EvaluationResult::Value(value) => value,
        EvaluationResult::Throw(err) => return EvaluationResult::Throw(err),
        EvaluationResult::Return(_) => {
            return EvaluationResult::Throw(Value::String(
                "A return statement is not valid in the value of an assignment.".to_string(),
            ));
        }
    };

    // Assign the value to the variable in the scope
    // TODO: Add utility function for assigning a variable in the scope and handling the result, since this will be a common operation.
    match scope
        .assign_variable(&assignment.identifier.id, value)
        .map_err(|error| Value::String(format!("Error assigning variable: {:?}", error)))
        .map(|_| Value::Nil)
    {
        Ok(value) => EvaluationResult::Value(value),
        Err(err) => EvaluationResult::Throw(err),
    }
}
