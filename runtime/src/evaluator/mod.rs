mod branches;
mod evaluation_result;

use crate::{Environment, Scope, Value};
use ast::{Expression, Literal, Program};
pub use branches::*;
pub use evaluation_result::EvaluationResult;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Evaluator {
    environment: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Self { environment }
    }

    /// Run all of the expressions in the program and return the value of the last expression.
    pub fn evaluate_program(&mut self, program: Program) -> Result<Value, Value> {
        let mut items = program.body.items;

        let mut scope = Scope::with_parent(Box::new(
            // TODO: This is a bit of a hack to get around the fact that the scope needs to be mutable,
            // but the environment is not. This should be refactored.
            self.environment.take().global_scope,
        ));

        // TODO: Add utility function for evaluating a sequence of expressions and returning the value of the last expression,
        // since this will be a common operation.
        match items.pop() {
            None => Ok(Value::Nil),
            Some(last) => {
                for expr in items {
                    match self.evaluate_expression(expr, &mut scope) {
                        EvaluationResult::Throw(err) => return Err(err),
                        _ => continue,
                    }
                }

                match self.evaluate_expression(last, &mut scope) {
                    EvaluationResult::Throw(err) => Err(err),
                    EvaluationResult::Value(x) | EvaluationResult::Return(x) => Ok(x),
                }
            }
        }
    }

    pub fn evaluate_expression(
        &mut self,
        expression: Expression,
        scope: &mut Scope,
    ) -> EvaluationResult {
        match expression {
            Expression::Assignment(_assignment) => todo!(),
            Expression::Block(_block) => todo!(),
            Expression::Dyadic(dyadic) =>
            // TODO: Move to own branch
            {
                let left_value = match self.evaluate_expression(*dyadic.left, scope) {
                    EvaluationResult::Value(value) => value,
                    EvaluationResult::Throw(err) => return EvaluationResult::Throw(err),
                    EvaluationResult::Return(value) => return EvaluationResult::Return(value),
                };

                let right_value = match self.evaluate_expression(*dyadic.right, scope) {
                    EvaluationResult::Value(value) => value,
                    EvaluationResult::Throw(err) => return EvaluationResult::Throw(err),
                    EvaluationResult::Return(value) => return EvaluationResult::Return(value),
                };

                match (left_value, right_value, dyadic.operator) {
                    (Value::Number(left), Value::Number(right), ast::DyadicOperator::Add) => {
                        EvaluationResult::Value(Value::Number(left + right))
                    }
                    (Value::Number(left), Value::Number(right), ast::DyadicOperator::Subtract) => {
                        EvaluationResult::Value(Value::Number(left - right))
                    }
                    (Value::Number(left), Value::Number(right), ast::DyadicOperator::Multiply) => {
                        EvaluationResult::Value(Value::Number(left * right))
                    }
                    (Value::Number(left), Value::Number(right), ast::DyadicOperator::Divide) => {
                        if right == 0. {
                            // TODO: This should be a custom error type instead of a string.
                            EvaluationResult::Throw(Value::String(
                                "Cannot divide by zero.".to_string(),
                            ))
                        } else {
                            EvaluationResult::Value(Value::Number(left / right))
                        }
                    }
                    // TODO: Support more operators and operand types.
                    (op, left, right) => EvaluationResult::Throw(Value::String(format!(
                        "Unsupported operands for operator {:?}: {:?} and {:?}",
                        op, left, right
                    ))),
                }
            }
            Expression::FunctionCall(_function_call) => todo!(),
            Expression::FunctionDeclaration(_function_declaration) => todo!(),
            Expression::Identifier(identifier) => evaluate_identifier(identifier, scope),
            Expression::Literal(literal) =>
            // TODO: Move to own branch
            {
                let value = match literal {
                    Literal::Integer(int) => Value::Number(int.value as f64),
                    Literal::String(string) => Value::String(string.value),
                    Literal::Boolean(boolean) => Value::Boolean(boolean.value),
                    Literal::Nil => Value::Nil,
                    Literal::Decimal(_decimal_literal) => todo!(),
                    Literal::Hexadecimal(_hexadecimal_literal) => todo!(),
                    Literal::Binary(_binary_literal) => todo!(),
                    Literal::Octal(_octal_literal) => todo!(),
                };

                EvaluationResult::Value(value)
            }
            Expression::Match(_match) => todo!(),
            Expression::Member(_member) => todo!(),
            Expression::Return(_return) => todo!(),
            Expression::VariableDeclaration(variable_declaration) =>
            // TODO: Move to own branch
            {
                let initial_value: Option<Value> = match variable_declaration.initial_value {
                    Some(initial) => match self.evaluate_expression(*initial, scope) {
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
                    .map_err(|error| Value::String(format!("Error adding variable: {:?}", error)))
                    .map(|_| Value::Nil)
                {
                    Ok(value) => EvaluationResult::Value(value),
                    Err(err) => EvaluationResult::Throw(err),
                }
            }
        }
    }
}
