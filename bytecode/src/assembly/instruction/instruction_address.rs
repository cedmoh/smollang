use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

use crate::InstructionOffset;

/// An absolute address of an instruction in the program
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstructionAddress(usize);

impl InstructionAddress {
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

    pub fn add_offset(&mut self, offset: InstructionOffset) {
        let isize = offset.as_isize();
        if isize.is_positive() {
            self.0 += isize as usize;
        } else {
            self.0 -= isize.abs() as usize;
        }
    }
}

impl From<usize> for InstructionAddress {
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

impl Display for InstructionAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>4}", self.0)
    }
}

impl Add<InstructionOffset> for InstructionAddress {
    type Output = Self;

    fn add(self, rhs: InstructionOffset) -> Self::Output {
        let mut result = self;
        result.add_offset(rhs);
        result
    }
}

impl AddAssign<InstructionOffset> for InstructionAddress {
    fn add_assign(&mut self, rhs: InstructionOffset) {
        self.add_offset(rhs)
    }
}
