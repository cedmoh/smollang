use crate::visitors::{
    AstToAssemblyVisitor, CompileVisitor, FatalCompilerError,
};
use ast::Program;
use bytecode::Assembly;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(
        &mut self,
        program: Program,
    ) -> Result<Assembly, FatalCompilerError> {
        let mut visitor = AstToAssemblyVisitor::new();

        visitor.visit(&program)?;

        Ok(visitor.assembly_builder.build())
    }
}
