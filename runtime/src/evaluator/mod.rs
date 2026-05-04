mod branches;
mod evaluation_result;
mod util;

use crate::{Environment, Scope};
use ast::{Expression, Program};
pub use branches::*;
pub use evaluation_result::EvaluationResult;
use std::{cell::RefCell, rc::Rc};
pub use util::evaluate_expressions_and_return_last_value;

#[derive(Debug)]
pub struct Evaluator {
    environment: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new(environment: Rc<RefCell<Environment>>) -> Self {
        Self { environment }
    }

    /// Run all of the expressions in the program and return the value of the last expression.
    pub fn evaluate_program(&mut self, program: Program) -> EvaluationResult {
        let mut scope = Scope::with_parent(Box::new(
            // TODO: This is a bit of a hack to get around the fact that the scope needs to be mutable,
            // but the environment is not. This should be refactored.
            self.environment.take().global_scope,
        ));

        evaluate_expressions_and_return_last_value(program.body.items, self, &mut scope)
    }

    pub fn evaluate_expression(
        &self,
        expression: Expression,
        mut scope: &mut Scope,
    ) -> EvaluationResult {
        match expression {
            Expression::Assignment(assignment) => {
                evaluate_assignment(assignment, &self, &mut scope)
            }

            Expression::Block(block) => evaluate_block(block, &self, &mut scope),
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
