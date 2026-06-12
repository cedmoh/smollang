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

    pub fn load(&self, addr: MemoryAddress) -> Result<Value, MemoryError> {
        if addr >= self.data.len() {
            return Err(MemoryError::OutOfBounds(addr));
        }

        Ok(self.data[addr].clone())
    }

    pub fn store(
        &mut self,
        addr: MemoryAddress,
        value: Value,
    ) -> Result<(), MemoryError> {
        if addr >= self.data.len() {
            return Err(MemoryError::OutOfBounds(addr));
        }

        self.data[addr] = value;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Attempted to access memory out of bounds: {0}")]
    OutOfBounds(MemoryAddress),

    #[error("Attempted to access uninitialized memory at address: {0}")]
    UninitializedMemoryAccess(MemoryAddress),
}

pub type MemoryAddress = usize;
