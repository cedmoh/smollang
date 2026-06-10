use crate::{
    call_stack::CallStackAddress, memory::MemoryAddress,
    program::ProgramAddress, value::Value,
};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum Instruction {
    // Stack
    /// Push a value onto the stack
    Push(Value),
    /// Pop a value from the stack
    Pop,
    /// Duplicate the top value on the stack
    Dup,

    // Arithmetic
    /// Add the top two values on the stack and push the result back onto the
    /// stack
    Add,
    /// Subtract the top two values on the stack and push the result back onto
    /// the stack
    Sub,
    /// Multiply the top two values on the stack and push the result back onto
    /// the stack
    Mul,
    /// Divide the top two values on the stack and push the result back onto the
    /// stack
    Div,

    // Comparison
    /// Compare the top two values on the stack for equality and push the result
    /// (a boolean) back onto the stack
    Eq,
    /// Compare the top two values on the stack for less than and push the
    /// result (a boolean) back onto the stack
    Lt,
    /// Compare the top two values on the stack for greater than and push the
    /// result (a boolean) back onto the stack
    Gt,

    // Control flow
    /// Unconditionally jump to the instruction at the given address
    Jump(ProgramAddress),
    /// Jump to the instruction at the given address if the top value on the
    /// stack is true (non-zero for integers)
    JumpIfTrue(ProgramAddress),
    /// Jump to the instruction at the given address if the top value on the
    /// stack is false (zero for integers)
    JumpIfFalse(ProgramAddress),

    // Memory
    /// Load a value from memory at the given address and push it onto the stack
    Load(MemoryAddress),
    Store(MemoryAddress),

    // Functions
    Call(CallStackAddress),
    Ret,

    // Debug
    Print,

    Halt,
}
