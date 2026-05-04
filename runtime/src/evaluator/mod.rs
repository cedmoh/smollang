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
        &self,
        expression: Expression,
        mut scope: &mut Scope,
    ) -> EvaluationResult {
        match expression {
            Expression::Assignment(_assignment) => todo!(),
            Expression::Block(_block) => todo!(),
            Expression::Dyadic(dyadic) => evaluate_dyadic(dyadic, &self, &mut scope),
            Expression::FunctionCall(_function_call) => todo!(),
            Expression::FunctionDeclaration(_function_declaration) => todo!(),
            Expression::Identifier(identifier) => evaluate_identifier(identifier, &self, &scope),
            Expression::Literal(literal) => evaluate_literal(literal),
            Expression::Match(_match) => todo!(),
            Expression::Member(_member) => todo!(),
            Expression::Return(_return) => todo!(),
            Expression::VariableDeclaration(variable_declaration) => {
                evaluate_variable_declaration(variable_declaration, &self, &mut scope)
            }
        }
    }
}
