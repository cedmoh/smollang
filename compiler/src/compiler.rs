use crate::visitors::{AstToInstructionVisitor, Visitor};
use ast::Program;
use bytecode::Instruction;
use std::collections::HashMap;

pub struct Compiler {
    symbol_table: HashMap<String, ()>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn compile(&mut self, program: Program) -> Vec<Instruction> {
        let mut visitor = AstToInstructionVisitor::new();

        visitor.visit(&program);

        visitor.instructions
    }
}
