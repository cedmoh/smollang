use crate::{Constants, Instructions};

#[derive(Debug, Clone)]
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
    pub fn new() -> Self {
        Self {
            instructions: Instructions::new(),
            constants: Constants::new(),
        }
    }

    pub fn with_instructions(
        mut self,
        instructions: impl Into<Instructions>,
    ) -> Self {
        self.instructions = instructions.into();
        self
    }

    pub fn with_constants(mut self, constants: impl Into<Constants>) -> Self {
        self.constants = constants.into();
        self
    }
}
