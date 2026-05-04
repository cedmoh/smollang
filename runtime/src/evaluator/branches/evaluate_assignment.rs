use ast::Assignment;

use crate::{EvaluationResult, Evaluator, Scope, Value};

pub fn evaluate_assignment(
    assignment: Assignment,
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

#[cfg(test)]
mod tests {
    use crate::{Environment, ScopeItem};

    use super::*;
    use ast::*;

    #[test]
    fn should_assign_new_value() {
        // Arrange
        let variable_name = "x".to_string();
        let variable_value = 3.0;
        let variable_value_to_assign = 5.0;

        // TODO: Add utility function for creating an environment, scope and evaluator for testing, since this will be a common setup.
        let environment = Environment::new();
        let mut scope = Scope::new();
        scope
            .add_variable(&variable_name, Value::Number(variable_value), true)
            .unwrap();
        let evaluator = &Evaluator::new(std::rc::Rc::new(std::cell::RefCell::new(environment)));

        // Act
        let result = evaluate_assignment(
            Assignment::new(
                Identifier::new(variable_name.clone()),
                Box::new(Expression::Literal(Literal::Decimal(DecimalLiteral::new(
                    variable_value_to_assign,
                )))),
            ),
            evaluator,
            &mut scope,
        );

        // Assert
        assert!(result.is_ok());

        let Some(ScopeItem::Variable { value, .. }) = scope.lookup_current_scope(&variable_name)
        else {
            panic!("Expected variable to have been added to scope when preparing the test.")
        };

        assert_eq!(value, &Value::Number(variable_value_to_assign));
    }
}
