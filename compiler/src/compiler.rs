use crate::visitors::{
    AstToAssemblyVisitor, AstToAssemblyVisitorError, Visitor,
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
    ) -> Result<Assembly, AstToAssemblyVisitorError> {
        let mut visitor = AstToAssemblyVisitor::new();

        visitor.visit(&program)?;

        Ok(visitor.assembly_builder.build())
    }
}
