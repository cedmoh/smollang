use crate::{
    EvaluationResult::{self, Return, Throw, Value},
    Evaluator, Scope, Value as RuntimeValue,
};

/// Evaluate a list of expressions and return the value of the last expression, or an error if any of the expressions throw an error.
pub fn evaluate_expressions_and_return_last_value(
    mut expressions: Vec<ast::Expression>,
    evaluator: &Evaluator,
    scope: &mut Scope,
) -> EvaluationResult {
    match expressions.pop() {
        None => Value(RuntimeValue::Nil),
        Some(last) => {
            for expr in expressions {
                match evaluator.evaluate_expression(expr, scope) {
                    Throw(err) => return Throw(err),
                    // If any expression in the block returns a value, we handle it by breaking out of the block
                    // and returning the value.
                    Return(value) => return Value(value),
                    Value(_) => continue,
                }
            }

            match evaluator.evaluate_expression(last, scope) {
                Throw(err) => Throw(err),
                // If the last expression in the block returns a value, we return that value as the value of the block.
                Return(value) | Value(value) => Value(value),
            }
        }
    }
}
