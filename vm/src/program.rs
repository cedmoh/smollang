use crate::Instruction;

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    pub fn empty() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn get(&self, addr: ProgramAddress) -> Instruction {
        self.instructions[addr.as_usize()]
    }
}

impl From<Vec<Instruction>> for Program {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}

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
}

impl From<usize> for ProgramAddress {
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}
