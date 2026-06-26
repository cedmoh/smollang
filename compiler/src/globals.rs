use std::collections::HashMap;

use bytecode::ConstantAddress;

pub struct Globals(HashMap<String, ConstantAddress>);

impl Globals {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, name: String, address: ConstantAddress) {
        self.0.insert(name, address);
    }

    pub fn get(&self, name: &str) -> Option<&ConstantAddress> {
        self.0.get(name)
    }
}
