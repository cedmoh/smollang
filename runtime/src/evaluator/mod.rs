use ast::Program;

use crate::Value;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate_program(&mut self, program: Program) -> Value {
        Value::Nil
    }

    pub fn evaluate_expression(&mut self, expression: ast::Expression) -> Value {
        Value::Nil
    }
}
