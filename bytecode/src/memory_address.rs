use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MemoryAddress(usize);

impl MemoryAddress {
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for MemoryAddress {
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl Display for MemoryAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>4}", self.0)
    }
}
