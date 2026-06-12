use std::{fmt::Display, ops::Index};

use bytecode::{
    Instruction::{self, Halt},
    ProgramAddress,
};

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            instructions: vec![Halt],
        }
    }

    pub fn with_instructions(instructions: Vec<Instruction>) -> Self {
        let maybe_last_instruction = instructions.last();

        let Some(last_instruction) = maybe_last_instruction else {
            return Self::new();
        };

        // If the last instruction is not a HALT, we need to append one to ensure the
        // program never risks running off the end of the instruction list.
        // TODO: Move this logic to the compiler
        match last_instruction {
            Halt => Self { instructions },
            _ => Self {
                instructions: [instructions, vec![Halt]].concat(),
            },
        }
    }
}

impl Index<ProgramAddress> for Program {
    type Output = Instruction;

    fn index(&self, index: ProgramAddress) -> &Self::Output {
        &self.instructions[index.as_usize()]
    }
}

impl From<Vec<Instruction>> for Program {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "{:0>4} {}", i, instruction)?;
        }

        Ok(())
    }
}
