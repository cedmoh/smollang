use crate::call_stack::CallStack;
use crate::io::{DummyIo, DummyIoError, Io};
use crate::memory::{Memory, MemoryError};
use crate::value_stack::{ValueStack, ValueStackError};
use bytecode::{
    Assembly, Instruction, InstructionAddress, Object, ObjectData, Value,
};
use thiserror::Error;

pub struct Vm<IoError> {
    /// Points to the current instruction being executed
    pub instruction_pointer: InstructionAddress,

    /// Used for storing intermediate values during execution
    pub stack: ValueStack,

    /// Used for storing return addresses when calling functions
    call_stack: CallStack,

    /// Used for storing values in a simple fixed-size memory
    memory: Memory,

    /// The program, which is a list of instructions to be executed
    assembly: Assembly,

    /// used for standard input and output (e.g. for the `Print` instruction)
    pub io: Box<dyn Io<IoError>>,
}

impl Vm<DummyIoError> {
    pub fn new() -> Self {
        Self {
            instruction_pointer: InstructionAddress::zero(),
            stack: ValueStack::new(),
            call_stack: CallStack::new(),
            memory: Memory::new(),
            assembly: Assembly::new(),
            io: Box::new(DummyIo::new()),
        }
    }
}

impl<IoError> Vm<IoError> {
    pub fn new_with_io(io: Box<dyn Io<IoError>>) -> Self {
        Self {
            instruction_pointer: InstructionAddress::zero(),
            stack: ValueStack::new(),
            call_stack: CallStack::new(),
            memory: Memory::new(),
            assembly: Assembly::new(),
            io,
        }
    }

    pub fn load_assembly(&mut self, assembly: Assembly) -> &mut Self {
        self.assembly = assembly;
        self
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        use Instruction::*;
        use VmError::*;

        loop {
            let instr = self.assembly.instructions[self.instruction_pointer];

            match instr {
                // Stack
                Push(value) => {
                    self.stack.push(value);
                    self.instruction_pointer.increment();
                }
                Pop => {
                    self.stack.pop()?;
                    self.instruction_pointer.increment();
                }
                Duplicate => {
                    self.stack.duplicate()?;
                    self.instruction_pointer.increment();
                }
                DuplicateTwo => {
                    self.stack.duplicate_two()?;
                    self.instruction_pointer.increment();
                }

                // Arithmetic
                Add => {
                    use Value::*;

                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack.push(Int(lhs + rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be added.
                        _ => unreachable!(),
                    }

                    self.instruction_pointer.increment();
                }
                Subtract => {
                    use Value::*;

                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack.push(Int(lhs - rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be subtracted.
                        _ => unreachable!(),
                    }

                    self.instruction_pointer.increment();
                }
                Multiply => {
                    use Value::*;

                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack.push(Int(lhs * rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be multiplied.
                        _ => unreachable!(),
                    }

                    self.instruction_pointer.increment();
                }
                Divide => {
                    use Value::*;

                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack.push(Int(lhs / rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be divided.
                        _ => unreachable!(),
                    }

                    self.instruction_pointer.increment();
                }

                // Comparison
                Equals => {
                    use Value::*;

                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack.push(Boolean(lhs == rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be compared.
                        _ => unreachable!(),
                    }

                    self.instruction_pointer.increment();
                }
                LessThan => {
                    use Value::*;

                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack.push(Boolean(lhs < rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be compared.
                        _ => unreachable!(),
                    }

                    self.instruction_pointer.increment();
                }
                GreaterThan => {
                    use Value::*;

                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;

                    match (a, b) {
                        (Int(lhs), Int(rhs)) => {
                            self.stack.push(Boolean(lhs > rhs));
                        }
                        // The usage of the VM should ensure that only integers
                        // are attempted to be compared.
                        _ => unreachable!(),
                    }

                    self.instruction_pointer.increment();
                }
                NotEquals => todo!(),
                LessThanOrEqual => todo!(),
                GreaterThanOrEqual => todo!(),

                // Control flow
                Jump(addr) => {
                    self.instruction_pointer += addr;
                }
                JumpIfTrue(addr) => {
                    use Value::*;

                    let cond = self.stack.pop()?;
                    match cond {
                        Boolean(true) => {
                            self.instruction_pointer += addr;
                        }
                        Boolean(false) => {
                            self.instruction_pointer.increment();
                        }
                        // The usage of the VM should ensure that only boolean
                        // values are used for
                        // conditional jumps.
                        _ => unreachable!(),
                    }
                }
                JumpIfFalse(addr) => {
                    use Value::*;

                    let cond = self.stack.pop()?;
                    match cond {
                        Boolean(true) => {
                            self.instruction_pointer.increment();
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
                    self.stack.push(Value::Object(
                        // Cast the memory address to an object handle
                        // The memory address should always be a valid object
                        // handle
                        addr.cast_to_object_handle(),
                    ));

                    self.instruction_pointer.increment();
                }
                Store(_addr) => {
                    todo!("The STORE instruction is not yet implemented");
                }

                // Constants
                Constant(addr) => {
                    use bytecode::Constant;

                    // Clone the constant from the assembly's constant pool and
                    // convert it into a value that can be
                    // pushed onto the stack.
                    let value = match self.assembly.constants[addr].clone() {
                        Constant::Nil => Value::Nil,
                        Constant::Int(i) => Value::Int(i),
                        Constant::Boolean(b) => Value::Boolean(b),
                        Constant::Float(_) => {
                            todo!("Float constants are not yet supported")
                        }
                        Constant::String(s) => Value::Object(
                            // Cast the memory address to an object handle
                            // because we know that the memory address returned
                            // by `store` will always be a valid object handle.
                            self.memory
                                .store(Object::new_string(s))?
                                .cast_to_object_handle(),
                        ),
                    };

                    self.stack.push(value);
                    self.instruction_pointer.increment();
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

                // IO
                In => {
                    todo!("The IN instruction is not yet implemented");
                }
                Out => {
                    let value = self.stack.pop()?;

                    match value {
                        Value::Object(object_handle) => {
                            let object =
                                self.memory.load(object_handle.into())?;

                            match object.data {
                                ObjectData::String(s) => {
                                    self.io.write_line(&s.0);
                                }
                            }
                        }
                        x => {
                            self.io.write_line(&x.to_string());
                        }
                    }

                    self.instruction_pointer.increment();
                }

                Halt => break Ok(()),
                _ => todo!(),
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum VmError {
    #[error("Value stack error: {0}")]
    StackError(#[from] ValueStackError),

    #[error("Memory error: {0}")]
    MemoryError(#[from] MemoryError),

    #[error("Attempted to pop return address from empty call stack")]
    CallStackUnderflow,
}
