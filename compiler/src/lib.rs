use std::collections::HashMap;

use ast::Program;

pub trait Visitor<T> {
    fn visit(&mut self, program: &T) -> T;
}

pub struct Compiler {
    symbol_table: HashMap<String, ()>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn compile(&mut self, program: &Program) -> Vec<()> {
        todo!()
    }
}
