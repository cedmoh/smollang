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
    Finished(Value),

    /// The program threw a runtime error.
    RuntimeError(Value),
    // TODO: Add a variant for when the program panics,
    // which is different from a runtime error since it indicates a bug in the interpreter rather than an error in the program being run.
}

impl From<EvaluationResult> for RunResult {
    fn from(evaluation_result: EvaluationResult) -> Self {
        match evaluation_result {
            EvaluationResult::Throw(err) => RunResult::RuntimeError(err),
            EvaluationResult::Return(value) | EvaluationResult::Value(value) => {
                RunResult::Finished(value)
            }
        }
    }
}

impl RunResult {
    pub fn is_finished(&self) -> bool {
        matches!(self, RunResult::Finished(_))
    }

    pub fn is_runtime_error(&self) -> bool {
        matches!(self, RunResult::RuntimeError(_))
    }

    pub fn expect_finished(self, msg: &str) -> Value {
        match self {
            RunResult::Finished(value) => value,
            RunResult::RuntimeError(err) => panic!("{}: {:?}", msg, err),
        }
    }
}
