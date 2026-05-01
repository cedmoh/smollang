use crate::{Environment, Evaluator, Value};
use ast::Program;

pub struct Runner {
    environment: Environment,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn run(&mut self, program: Program) -> Value {
        Evaluator::new().evaluate_program(program)
    }
}
