use ast::Program;
use bytecode::FunctionObject;

use crate::compilation_visitor::{
    CompilationVisitor, FatalCompilationError, VisitAndCompile,
};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(
        &mut self,
        program: Program,
    ) -> Result<FunctionObject, FatalCompilationError> {
        let mut visitor = CompilationVisitor::new();

        visitor.visit(&program)?;

        visitor.build()
    }
}
