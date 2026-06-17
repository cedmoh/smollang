use crate::visitors::{AstToAssemblyVisitor, Visitor};
use ast::Program;
use bytecode::Assembly;
use std::collections::HashMap;

pub struct Compiler {
    _symbol_table: HashMap<String, ()>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            _symbol_table: HashMap::new(),
        }
    }

    pub fn compile(&mut self, program: Program) -> Assembly {
        let mut visitor = AstToAssemblyVisitor::new();

        visitor.visit(&program);

        visitor.assembly_builder.build()
    }
}
