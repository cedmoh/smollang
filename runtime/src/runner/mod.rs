use crate::{Environment, Evaluator, Value};
use ast::Program;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Runner {
    pub environment: Rc<RefCell<Environment>>,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn with_environment(environment: Environment) -> Self {
        Self {
            environment: Rc::new(RefCell::new(environment)),
        }
    }

    pub fn run(&self, program: Program) -> Result<Value, Value> {
        Evaluator::new(self.environment.clone()).evaluate_program(program)
    }
}
