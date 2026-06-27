use bytecode::{MemoryAddress, Object};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Memory {
    /// The collection of cells in memory that can hold objects.
    /// Each cell can either be occupied by an object or be empty (None).
    data: Vec<Option<Object>>,

    /// A list of free memory addresses that can be reused for storing new
    /// objects.
    free: Vec<usize>,
}

const MEMORY_CAPACITY: usize = 32;

impl Memory {
    pub fn new() -> Self {
        Self {
            data: vec![None; MEMORY_CAPACITY],
            free: (0..MEMORY_CAPACITY).rev().collect(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: vec![None; capacity],
            free: (0..capacity).rev().collect(),
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

    pub fn dump(self) -> Vec<Option<Object>> {
        self.data
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
