use std::{fmt::Display, ops::Index};

use crate::{Instruction, InstructionAddress};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.0.push(instruction);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Instruction> {
        self.0.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<Instruction>> for Instructions {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self(instructions)
    }
}

impl Into<Vec<Instruction>> for Instructions {
    fn into(self) -> Vec<Instruction> {
        self.0
    }
}

impl Index<InstructionAddress> for Instructions {
    type Output = Instruction;

    fn index(&self, index: InstructionAddress) -> &Self::Output {
        &self.0[index.as_usize()]
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instruction in self.0.iter() {
            writeln!(f, "{}", instruction)?;
        }

        Ok(())
    }
}
