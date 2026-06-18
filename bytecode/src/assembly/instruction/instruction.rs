use std::fmt::Display;

use crate::{ConstantAddress, InstructionAddress, InstructionOffset, Value};

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
    Load(ConstantAddress),
    /// Pop a value from the stack and store it in memory at the given address
    Store(ConstantAddress),

    // Functions
    Call(InstructionAddress),
    Return,

    // Constants
    /// Push a constant value from the constant pool onto the stack
    Constant(ConstantAddress),

    Halt,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;

        match self {
            Push(value) => write!(f, "PUSH {}", value),
            Pop => write!(f, "POP"),
            Duplicate => write!(f, "DUP"),
            Add => write!(f, "ADD"),
            Subtract => write!(f, "SUB"),
            Multiply => write!(f, "MUL"),
            Divide => write!(f, "DIV"),
            Equals => write!(f, "EQ"),
            NotEquals => write!(f, "NEQ"),
            LessThan => write!(f, "LT"),
            LessThanOrEqual => write!(f, "LEQ"),
            GreaterThan => write!(f, "GT"),
            GreaterThanOrEqual => write!(f, "GEQ"),
            Jump(program_offset) => {
                write!(f, "JMP {}", program_offset)
            }
            JumpIfTrue(program_offset) => {
                write!(f, "JT {}", program_offset)
            }
            JumpIfFalse(program_offset) => {
                write!(f, "JF {}", program_offset)
            }
            Load(memory_address) => {
                write!(f, "LOAD {}", memory_address)
            }
            Store(memory_address) => {
                write!(f, "STORE {}", memory_address)
            }
            Call(call_stack_address) => {
                write!(f, "CALL {}", call_stack_address)
            }
            Return => write!(f, "RET"),
            Halt => write!(f, "HALT"),
            DuplicateTwo => write!(f, "DUP2"),
            Constant(constant_address) => {
                write!(f, "CONST {}", constant_address)
            }
        }
    }
}
