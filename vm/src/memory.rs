use thiserror::Error;

use crate::value::Value;

#[derive(Debug)]
pub struct Memory {
    data: Vec<Value>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            // Initialize memory with 1024 cells
            data: vec![Value::Int(0); 1024],
        }
    }

    pub fn load(&self, addr: MemoryAddress) -> Option<Value> {
        self.data.get(addr).cloned()
    }

    pub fn store(
        &mut self,
        addr: MemoryAddress,
        value: Value,
    ) -> Result<(), MemoryError> {
        if addr < self.data.len() {
            self.data[addr] = value;
            Ok(())
        } else {
            Err(MemoryError::OutOfBounds(addr))
        }
    }
}

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Memory access out of bounds: {0}")]
    OutOfBounds(MemoryAddress),
}

pub type MemoryAddress = usize;
