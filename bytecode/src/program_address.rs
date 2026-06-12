use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use crate::ProgramOffset;

/// An absolute address of an instruction in the program
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramAddress(usize);

impl ProgramAddress {
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        self.0 -= 1;
    }

    pub fn add_offset(&mut self, offset: ProgramOffset) {
        self.0 += offset.as_usize();
    }
}

impl From<usize> for ProgramAddress {
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl Display for ProgramAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>4}", self.0)
    }
}

impl Add<ProgramOffset> for ProgramAddress {
    type Output = Self;

    fn add(self, rhs: ProgramOffset) -> Self::Output {
        Self(self.0 + rhs.as_usize())
    }
}

impl AddAssign<ProgramOffset> for ProgramAddress {
    fn add_assign(&mut self, rhs: ProgramOffset) {
        self.add_offset(rhs)
    }
}
