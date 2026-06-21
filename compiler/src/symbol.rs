use std::collections::HashMap;

use bytecode::ConstantAddress;

pub struct SymbolTable(HashMap<String, Symbol>);

impl SymbolTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, name: String, symbol: Symbol) {
        self.0.insert(name, symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        self.0.get(name)
    }
}

pub enum Symbol {
    Global(ConstantAddress),
}
