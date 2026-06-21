use std::fmt::Display;

use crate::{
    Constant, ConstantAddress, Constants,
    Instruction::{self, Halt},
    Instructions,
};

/// Represents a complete compiled program.
#[derive(Debug)]
pub struct Assembly {
    /// A list of instructions that make up the program.
    /// This is the main component of the assembly,
    /// as it represents the actual code that will be executed by the virtual
    /// machine.
    pub instructions: Instructions,

    /// A list of constant values used in the program.
    /// This is used to store literals (e.g., integers, booleans, strings)
    /// That are referenced by the `CONSTANT` instruction.
    pub constants: Constants,
}

impl Assembly {
    /// Creates a new, assembly with a single `Halt` instruction
    /// to prevent running off the end of the instruction list.
    pub fn new() -> Self {
        Self {
            instructions: vec![Halt].into(),
            constants: Vec::new().into(),
        }
    }

    pub fn builder() -> AssemblyBuilder {
        AssemblyBuilder::new()
    }
}

impl Display for Assembly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "== Program ==")?;
        writeln!(f, "{}", self.instructions)?;
        writeln!(f, "== Constants ==")?;
        writeln!(f, "{}", self.constants)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct AssemblyBuilder {
    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
}

impl AssemblyBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }

    pub fn add_instructions(
        &mut self,
        instructions: Vec<Instruction>,
    ) -> &mut Self {
        self.instructions.extend(instructions);
        self
    }

    pub fn instructions(mut self, instructions: Vec<Instruction>) -> Self {
        self.instructions = instructions;
        self
    }

    pub fn instruction_length(&self) -> usize {
        self.instructions.len()
    }

    pub fn add_constant(&mut self, constant: Constant) -> &mut Self {
        self.constants.push(constant);
        self
    }

    pub fn push_constant(&mut self, constant: Constant) -> ConstantAddress {
        self.constants.push(constant);
        ConstantAddress::new(self.constants.len() - 1)
    }

    pub fn constants(mut self, constants: Vec<Constant>) -> Self {
        self.constants = constants;
        self
    }

    pub fn build(self) -> Assembly {
        // Make sure the instruction list ends with a HALT instruction to
        // prevent running off the end of the list.
        let instructions = if self.instructions.ends_with(&[Halt]) {
            self.instructions
        } else {
            [self.instructions, vec![Halt]].concat()
        };

        Assembly {
            instructions: instructions.into(),
            constants: self.constants.into(),
        }
    }
}
