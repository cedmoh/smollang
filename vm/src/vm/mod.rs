mod runner;

use crate::call_stack::{CallFrame, CallStack};
use crate::global_environment::GlobalEnvironment;
use crate::io::{DummyIo, Io, IoError};
use crate::memory::{Memory, MemoryError};
use crate::value_stack::{ValueStack, ValueStackError};
use bytecode::{
    Assembly, Constant, ConstantAddress, InstructionAddress, InstructionOffset,
    Value,
};
use runner::run;
use thiserror::Error;

pub struct Vm<I>
where
    I: Io,
{
    /// Used for storing intermediate values during execution
    pub stack: ValueStack,

    /// Used for storing return addresses when calling functions
    pub frames: CallStack,

    /// Used for storing values in a simple fixed-size memory
    pub memory: Memory,

    /// The global environment table, which maps variable names to their values
    pub global_environment: GlobalEnvironment,

    /// used for standard input and output (e.g. for the `Print` instruction)
    pub io: I,
}

impl Vm<DummyIo> {
    pub fn new() -> Self {
        Self {
            stack: ValueStack::new(),
            frames: CallStack::new(),
            memory: Memory::new(),
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
            stack: ValueStack::new(),
            frames: CallStack::new(),
            memory: Memory::new(),
            global_environment: GlobalEnvironment::new(),
            io,
        }
    }

    pub fn load_assembly(
        &mut self,
        assembly: impl Into<Assembly>,
    ) -> &mut Self {
        self.current_frame_mut().function_object.chunk = assembly.into();
        self
    }

    fn current_frame(&self) -> &CallFrame {
        self.frames
            .last()
            .expect("Expected to always have at least one active frame")
    }

    fn current_frame_mut(&mut self) -> &mut CallFrame {
        self.frames
            .last_mut()
            .expect("Expected to always have at least one active frame")
    }

    fn increment_instruction_pointer(&mut self) {
        if let Some(frame) = self.frames.last_mut() {
            frame.instruction_pointer.increment();
        }
    }

    fn offset_instruction_pointer(&mut self, offset: InstructionOffset) {
        if let Some(frame) = self.frames.last_mut() {
            frame.instruction_pointer.add_offset(offset);
        }
    }

    fn get_current_instruction_address(&self) -> InstructionAddress {
        self.current_frame().instruction_pointer
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        if self.frames.last().is_none() {
            return Err(VmError::CallStackUnderflow);
        }

        run(self)
    }
}

#[derive(Debug, Error)]
pub enum VmError {
    #[error("Value stack error.")]
    StackError(#[from] ValueStackError),

    #[error("Memory error.")]
    MemoryError(#[from] MemoryError),

    /// Can be caused if there are no frames on the call stack
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
