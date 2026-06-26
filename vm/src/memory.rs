use bytecode::{MemoryAddress, Object};
use thiserror::Error;

#[derive(Debug)]
pub struct Memory {
    /// The collection of cells in memory that can hold objects.
    /// Each cell can either be occupied by an object or be empty (None).
    data: Vec<Option<Object>>,

    /// A list of free memory addresses that can be reused for storing new
    /// objects.
    free: Vec<usize>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            // Initialize memory with 1024 cells
            data: vec![None; 1024],
            free: (0..1024).rev().collect(),
        }
    }

    pub fn load(&self, address: MemoryAddress) -> Result<Object, MemoryError> {
        let address_usize = address.as_usize();

        if address_usize >= self.data.len() {
            return Err(MemoryError::OutOfBounds(address));
        }

        match &self.data[address_usize] {
            Some(object) => Ok(object.clone()),
            None => Err(MemoryError::UninitializedMemoryAccess(address)),
        }
    }

    pub fn store(
        &mut self,
        object: Object,
    ) -> Result<MemoryAddress, MemoryError> {
        match self.free.pop() {
            Some(free_address) => {
                self.data[free_address] = Some(object);
                Ok(MemoryAddress::new(free_address))
            }
            None => Err(MemoryError::OutOfMemory),
        }
    }
}

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Attempted to access memory out of bounds: {0}")]
    OutOfBounds(MemoryAddress),

    #[error("Memory is full, cannot store new object")]
    OutOfMemory,

    #[error("Attempted to access uninitialized memory at address: {0}")]
    UninitializedMemoryAccess(MemoryAddress),
}
