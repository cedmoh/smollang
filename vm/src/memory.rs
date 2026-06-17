use bytecode::{ConstantAddress, Value};
use thiserror::Error;

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

    pub fn load(&self, address: ConstantAddress) -> Result<Value, MemoryError> {
        let address_usize = address.as_usize();

        if address_usize >= self.data.len() {
            return Err(MemoryError::OutOfBounds(address));
        }

        Ok(self.data[address_usize].clone())
    }

    pub fn store(
        &mut self,
        address: ConstantAddress,
        value: Value,
    ) -> Result<(), MemoryError> {
        let address_usize = address.as_usize();

        if address_usize >= self.data.len() {
            return Err(MemoryError::OutOfBounds(address));
        }

        self.data[address_usize] = value;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Attempted to access memory out of bounds: {0}")]
    OutOfBounds(ConstantAddress),

    #[error("Attempted to access uninitialized memory at address: {0}")]
    UninitializedMemoryAccess(ConstantAddress),
}
