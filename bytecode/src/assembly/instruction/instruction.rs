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

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;

        let (mnemonic, operand) = match self {
            Push(value) => ("PUSH", Some(value.to_string())),
            Pop => ("POP", None),
            Duplicate => ("DUP", None),
            Add => ("ADD", None),
            Subtract => ("SUB", None),
            Multiply => ("MUL", None),
            Divide => ("DIV", None),
            Equals => ("EQ", None),
            NotEquals => ("NEQ", None),
            LessThan => ("LT", None),
            LessThanOrEqual => ("LEQ", None),
            GreaterThan => ("GT", None),
            GreaterThanOrEqual => ("GEQ", None),
            Jump(program_offset) => ("JMP", Some(program_offset.to_string())),
            JumpIfTrue(program_offset) => {
                ("JT", Some(program_offset.to_string()))
            }
            JumpIfFalse(program_offset) => {
                ("JF", Some(program_offset.to_string()))
            }
            Load(memory_address) => ("LOAD", Some(memory_address.to_string())),
            Store(memory_address) => {
                ("STORE", Some(memory_address.to_string()))
            }
            Call(call_stack_address) => {
                ("CALL", Some(call_stack_address.to_string()))
            }
            Return => ("RET", None),
            Halt => ("HALT", None),
            DuplicateTwo => ("DUP2", None),
            Constant(constant_address) => {
                ("CONST", Some(constant_address.to_string()))
            }
            In => ("IN", None),
            Out => ("OUT", None),
            SetGlobal(constant_address) => {
                ("SETGB", Some(constant_address.to_string()))
            }
            GetGlobal(constant_address) => {
                ("GETGB", Some(constant_address.to_string()))
            }
        };

        match operand {
            Some(op) => write!(f, "{:5} {}", mnemonic, op),
            None => write!(f, "{:5}  ", mnemonic),
        }
    }
}
