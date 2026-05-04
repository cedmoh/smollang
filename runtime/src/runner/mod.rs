use crate::{Environment, EvaluationResult, Evaluator, Value};
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

    pub fn run(&self, program: Program) -> RunResult {
        Into::<RunResult>::into(Evaluator::new(self.environment.clone()).evaluate_program(program))
    }
}

#[derive(Debug)]
pub enum RunResult {
    /// The program ran successfully and returned a value.
    Success(Value),

    /// The program panicked with an unhandled error.
    Panic(String),
}

impl From<EvaluationResult> for RunResult {
    fn from(evaluation_result: EvaluationResult) -> Self {
        match evaluation_result {
            EvaluationResult::Throw(err) => RunResult::Panic(format!("Runtime error: {:?}", err)),
            EvaluationResult::Return(value) | EvaluationResult::Value(value) => {
                RunResult::Success(value)
            }
        }
    }
}
