use thiserror::Error;

use crate::call_stack::CallStack;
use crate::instruction::Instruction;
use crate::io::{Io, StandardIo};
use crate::memory::Memory;
use crate::program::{Program, ProgramAddress};
use crate::value::Value;
use crate::value_stack::ValueStack;

pub struct Vm {
    /// Points to the current instruction being executed
    pub instruction_pointer: ProgramAddress,

    /// Used for storing intermediate values during execution
    pub stack: ValueStack,

    /// Used for storing return addresses when calling functions
    call_stack: CallStack,

    /// Used for storing values in a simple fixed-size memory
    memory: Memory,

    /// The program, which is a list of instructions to be executed
    program: Program,

    /// used for standard input and output (e.g. for the `Print` instruction)
    io: Box<dyn Io>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            instruction_pointer: ProgramAddress::zero(),
            stack: ValueStack::new(),
            call_stack: CallStack::new(),
            memory: Memory::new(),
            program: Program::new(),
            io: Box::new(StandardIo::new()),
        }
    }

    pub fn with_io(mut self, io: Box<dyn Io>) -> Self {
        self.io = io;
        self
    }

    pub fn load_program(&mut self, program: impl Into<Program>) -> &mut Self {
        self.program = program.into();
        self
    }

    fn increment_instruction_pointer(&mut self) {
        self.instruction_pointer.increment();
    }

    fn try_stack_pop(&mut self) -> Result<Value, VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }

    fn stack_push(&mut self, value: Value) {
        self.stack.push(value)
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        use Instruction::*;
        use Value::*;
        use VmError::*;

        loop {
            let instr = self.program[self.instruction_pointer];

            match instr {
                // Stack
                Push(value) => {
                    self.stack_push(value);
                    self.increment_instruction_pointer();
                }
                Pop => {
                    self.try_stack_pop()?;
                    self.increment_instruction_pointer();
                }
                Duplicate => {
                    let value =
                        self.stack.last().ok_or(DuplicateOnEmptyStack)?;
                    self.stack_push(value.clone());
                    self.increment_instruction_pointer();
                }

                // Arithmetic
                Add => {
                    let b = self.try_stack_pop()?;
                    let a = self.try_stack_pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack_push(Int(lhs + rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be added.
                        _ => unreachable!(),
                    }

                    self.increment_instruction_pointer();
                }
                Subtract => {
                    let b = self.try_stack_pop()?;
                    let a = self.try_stack_pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack_push(Int(lhs - rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be subtracted.
                        _ => unreachable!(),
                    }

                    self.increment_instruction_pointer();
                }
                Multiply => {
                    let b = self.try_stack_pop()?;
                    let a = self.try_stack_pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack_push(Int(lhs * rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be multiplied.
                        _ => unreachable!(),
                    }

                    self.increment_instruction_pointer();
                }
                Divide => {
                    let b = self.try_stack_pop()?;
                    let a = self.try_stack_pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack_push(Int(lhs / rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be divided.
                        _ => unreachable!(),
                    }

                    self.increment_instruction_pointer();
                }

                // Comparison
                Equals => {
                    let b = self.try_stack_pop()?;
                    let a = self.try_stack_pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack_push(Boolean(lhs == rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be compared.
                        _ => unreachable!(),
                    }

                    self.increment_instruction_pointer();
                }
                LessThan => {
                    let b = self.try_stack_pop()?;
                    let a = self.try_stack_pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack_push(Boolean(lhs < rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be compared.
                        _ => unreachable!(),
                    }

                    self.increment_instruction_pointer();
                }
                GreaterThan => {
                    let b = self.try_stack_pop()?;
                    let a = self.try_stack_pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack_push(Boolean(lhs > rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be compared.
                        _ => unreachable!(),
                    }

                    self.increment_instruction_pointer();
                }

                // Control flow
                Jump(addr) => {
                    self.instruction_pointer += addr;
                }
                JumpIfTrue(addr) => {
                    let cond = self.try_stack_pop()?;
                    match cond {
                        Boolean(true) => {
                            self.instruction_pointer += addr;
                        }
                        Boolean(false) => {
                            self.increment_instruction_pointer();
                        }
                        // The usage of the VM should ensure that only boolean
                        // values are used for
                        // conditional jumps.
                        _ => unreachable!(),
                    }
                }
                JumpIfFalse(addr) => {
                    let cond = self.try_stack_pop()?;
                    match cond {
                        Boolean(true) => {
                            self.increment_instruction_pointer();
                        }
                        Boolean(false) => {
                            self.instruction_pointer += addr;
                        }
                        // The usage of the VM should ensure that only boolean
                        // values are used for
                        // conditional jumps.
                        _ => unreachable!(),
                    }
                }

                // Memory
                Load(addr) => {
                    let value = self.memory.load(addr).unwrap();
                    self.stack_push(value);
                    self.increment_instruction_pointer();
                }
                Store(addr) => {
                    let value = self.try_stack_pop()?;
                    self.memory.store(addr, value).unwrap();
                    self.increment_instruction_pointer();
                }

                // Functions
                Call(addr) => {
                    let address_to_return_to_when_call_returns =
                        self.instruction_pointer.as_usize() + 1;

                    self.call_stack
                        .push(address_to_return_to_when_call_returns);

                    self.instruction_pointer = addr.into();
                }
                Return => {
                    let address_to_return_to =
                        self.call_stack.pop().ok_or(CallStackUnderflow)?;

                    self.instruction_pointer = address_to_return_to.into();
                }
                Halt => break Ok(()),
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum VmError {
    #[error("Attempted to duplicate value on empty stack")]
    DuplicateOnEmptyStack,

    #[error("Attempted to pop value from empty stack")]
    StackUnderflow,

    #[error("Attempted to pop return address from empty call stack")]
    CallStackUnderflow,
}
