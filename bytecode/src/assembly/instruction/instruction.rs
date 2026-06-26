use std::fmt::Display;

use crate::{
    ConstantAddress, InstructionAddress, InstructionOffset, MemoryAddress,
    Value,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Instruction {
    // Stack
    /// Push a value onto the stack
    Push(Value),
    /// Pop a value from the stack
    Pop,
    /// Duplicate the top value on the stack
    Duplicate,
    /// Duplicate the top two values on the stack
    DuplicateTwo,

    // Arithmetic
    /// Add the top two values on the stack and push the result back onto the
    /// stack
    Add,
    /// Subtract the top two values on the stack and push the result back onto
    /// the stack
    Subtract,
    /// Multiply the top two values on the stack and push the result back onto
    /// the stack
    Multiply,
    /// Divide the top two values on the stack and push the result back onto the
    /// stack
    Divide,

    // Comparison
    /// Compare the top two values on the stack for equality and push the result
    /// (a boolean) back onto the stack
    Equals,
    /// Compare the top two values on the stack for inequality and push the
    /// result (a boolean) back onto the stack
    NotEquals,
    /// Compare the top two values on the stack for less than and push the
    /// result (a boolean) back onto the stack
    LessThan,
    /// Compare the top two values on the stack for greater than and push the
    /// result (a boolean) back onto the stack
    GreaterThan,
    /// Compare the top two values on the stack for less than or equal and push
    /// the result (a boolean) back onto the stack
    LessThanOrEqual,
    /// Compare the top two values on the stack for greater than or equal and
    /// push the result (a boolean) back onto the stack
    GreaterThanOrEqual,

    // Control flow
    /// Unconditionally jump to the instruction at the given address
    Jump(InstructionOffset),
    /// Jump to the instruction at the given address if the top value on the
    /// stack is true (non-zero for integers)
    JumpIfTrue(InstructionOffset),
    /// Jump to the instruction at the given address if the top value on the
    /// stack is false (zero for integers)
    JumpIfFalse(InstructionOffset),

    // Memory
    /// Load a value from memory at the given address and push it onto the stack
    Load(MemoryAddress),
    /// Pop a value from the stack and store it in memory at the given address
    Store(MemoryAddress),

    /// Set a local variable at the given slot index from the top stack value.
    /// This instruction does not pop the stack value.
    SetLocal(MemoryAddress),
    /// Get a local variable from the given slot index and push it onto the
    /// stack.
    GetLocal(MemoryAddress),

    /// Pop a value from the stack, store it in the globals table.
    /// The string name of the global variable is stored in the constant pool at
    /// the given address.
    SetGlobal(ConstantAddress),
    /// Look up a global variable by its name (stored in the constant pool at
    /// the given address), and push its value onto the stack
    GetGlobal(ConstantAddress),

    // Functions
    Call(InstructionAddress),
    Return,

    // IO
    /// Read a character from the standard input and push it onto the stack
    In,
    /// Pop a value from the stack and write it to the standard output
    Out,

    // Constants
    /// Push a constant value from the constant pool onto the stack
    Constant(ConstantAddress),

    Halt,
}

impl Instruction {
    pub fn name(&self) -> &'static str {
        use Instruction::*;

        match self {
            Push(_) => "PUSH",
            Pop => "POP",
            Duplicate => "DUP",
            DuplicateTwo => "DUP2",
            Add => "ADD",
            Subtract => "SUB",
            Multiply => "MUL",
            Divide => "DIV",
            Equals => "EQ",
            NotEquals => "NEQ",
            LessThan => "LT",
            GreaterThan => "GT",
            LessThanOrEqual => "LEQ",
            GreaterThanOrEqual => "GEQ",
            Jump(_) => "JMP",
            JumpIfTrue(_) => "JT",
            JumpIfFalse(_) => "JF",
            Load(_) => "LOAD",
            Store(_) => "STORE",
            SetLocal(_) => "SETLC",
            GetLocal(_) => "GETLC",
            SetGlobal(_) => "SETGB",
            GetGlobal(_) => "GETGB",
            Call(_) => "CALL",
            Return => "RET",
            In => "IN",
            Out => "OUT",
            Constant(_) => "CONST",
            Halt => "HALT",
        }
    }

    pub fn parameter_to_string(&self) -> Option<String> {
        use Instruction::*;

        match self {
            Push(value) => Some(value.to_string()),
            Jump(program_offset) => Some(program_offset.to_string()),
            JumpIfTrue(program_offset) => Some(program_offset.to_string()),
            JumpIfFalse(program_offset) => Some(program_offset.to_string()),
            Load(memory_address) => Some(memory_address.to_string()),
            Store(memory_address) => Some(memory_address.to_string()),
            SetLocal(memory_address) => Some(memory_address.to_string()),
            GetLocal(memory_address) => Some(memory_address.to_string()),
            Call(call_stack_address) => Some(call_stack_address.to_string()),
            Constant(constant_address) => Some(constant_address.to_string()),
            SetGlobal(constant_address) => Some(constant_address.to_string()),
            GetGlobal(constant_address) => Some(constant_address.to_string()),
            // Instructions without operands
            Pop | Duplicate | DuplicateTwo | Add | Subtract | Multiply
            | Divide | Equals | NotEquals | LessThan | GreaterThan
            | LessThanOrEqual | GreaterThanOrEqual | Return | In | Out
            | Halt => None,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.parameter_to_string() {
            None => write!(f, "{}", self.name()),
            Some(operand) => write!(f, "{} {}", self.name(), operand),
        }
    }
}
