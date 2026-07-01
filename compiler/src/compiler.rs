use ast::Program;
use bytecode::Assembly;

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
    ) -> Result<Assembly, FatalCompilationError> {
        let mut visitor = CompilationVisitor::new();

        visitor.visit(&program)?;

        visitor.build()
    }
}
