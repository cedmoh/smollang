use bytecode::Value;
use std::collections::HashMap;

pub struct GlobalEnvironment {
    /// The global environment table, which maps variable names to their values
    pub table: HashMap<String, Value>,
}

impl GlobalEnvironment {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, value: Value) {
        self.table.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.table.get(name)
    }
}
