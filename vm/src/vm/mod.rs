mod runner;

use crate::call_stack::CallStack;
use crate::global_environment::GlobalEnvironment;
use crate::io::{DummyIo, Io, IoError};
use crate::memory::{Memory, MemoryError};
use crate::value_stack::{ValueStack, ValueStackError};
use bytecode::{
    Assembly, Constant, ConstantAddress, InstructionAddress, Value,
};
use runner::run;
use thiserror::Error;

pub struct Vm<I>
where
    I: Io,
{
    /// Points to the current instruction being executed
    pub instruction_pointer: InstructionAddress,

    /// Used for storing intermediate values during execution
    pub stack: ValueStack,

    /// Used for storing return addresses when calling functions
    call_stack: CallStack,

    /// Used for storing values in a simple fixed-size memory
    pub memory: Memory,

    /// The program, which is a list of instructions to be executed
    assembly: Assembly,

    /// The global environment table, which maps variable names to their values
    global_environment: GlobalEnvironment,

    /// used for standard input and output (e.g. for the `Print` instruction)
    pub io: I,
}

impl Vm<DummyIo> {
    pub fn new() -> Self {
        Self {
            instruction_pointer: InstructionAddress::zero(),
            stack: ValueStack::new(),
            call_stack: CallStack::new(),
            memory: Memory::new(),
            assembly: Assembly::new(),
            global_environment: GlobalEnvironment::new(),
            io: DummyIo::new(),
        }
    }
}

impl<I> Vm<I>
where
    I: Io,
{
    pub fn new_with_io(io: I) -> Self {
        Self {
            instruction_pointer: InstructionAddress::zero(),
            stack: ValueStack::new(),
            call_stack: CallStack::new(),
            memory: Memory::new(),
            assembly: Assembly::new(),
            global_environment: GlobalEnvironment::new(),
            io,
        }
    }

    pub fn load_assembly(&mut self, assembly: Assembly) -> &mut Self {
        self.assembly = assembly;
        self
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        run(self)
    }
}

#[derive(Debug, Error)]
pub enum VmError {
    #[error("Value stack error.")]
    StackError(#[from] ValueStackError),

    #[error("Memory error.")]
    MemoryError(#[from] MemoryError),

    #[error("Attempted to pop return address from empty call stack")]
    CallStackUnderflow,

    /// Returned when the VM attempts to access a constant at an address that
    /// does not exist in the assembly's constant pool.
    #[error("Attempted to access constant at invalid address: {0}")]
    InvalidConstantAddress(ConstantAddress),

    /// Returned when the VM attempts to access a constant at an address that
    /// exists in the assembly's constant pool, but the constant at that address
    /// is not of the expected type.
    #[error("Expected constant at address {0} to be a {1}, but it was a {2}")]
    InvalidConstantType(ConstantAddress, &'static str, Constant),

    #[error("IO error.")]
    IoError(#[from] IoError),

    #[error("Arithmetic error.")]
    ArithmeticError(#[from] ArithmeticError),

    #[error("Dummy instruction offset encountered.")]
    DummyInstructionOffset,
}

#[derive(Debug, Error)]
pub enum ArithmeticError {
    #[error("Cannot add values {0} and {1}")]
    UnsupportedAdditionOperands(Value, Value),

    #[error("Cannot modulo values {0} and {1}")]
    UnsupportedModuloOperands(Value, Value),

    #[error("Cannot exponentiate values {0} and {1}")]
    UnsupportedPowerOperands(Value, Value),
}
