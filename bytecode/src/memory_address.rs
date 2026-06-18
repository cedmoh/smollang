use std::fmt::Display;

use crate::ObjectHandle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryAddress(usize);

impl MemoryAddress {
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn cast_to_object_handle(&self) -> ObjectHandle {
        ObjectHandle::new(*self)
    }
}

impl From<usize> for MemoryAddress {
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl From<ObjectHandle> for MemoryAddress {
    fn from(handle: ObjectHandle) -> Self {
        handle.into_memory_address()
    }
}

impl Display for MemoryAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>4}", self.0)
    }
}
