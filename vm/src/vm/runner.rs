use crate::Vm;
use crate::io::{Io, ToIoString};
use crate::value_stack::ValueStackError;
use crate::vm::{ArithmeticError as MathError, VmError};
use bytecode::{Instruction, InstructionOffset, Object, ObjectData, Value};

/// Runs the given VM until it halts or encounters an error.
pub fn run<I: Io>(vm: &mut Vm<I>) -> Result<(), VmError> {
    use Instruction::*;
    use VmError::*;

    loop {
        let instr = vm.assembly.instructions[vm.instruction_pointer];

        match instr {
            // Stack
            Push(value) => {
                vm.stack.push(value);
                vm.instruction_pointer.increment();
            }
            Pop => {
                vm.stack.pop()?;
                vm.instruction_pointer.increment();
            }
            Duplicate => {
                vm.stack.duplicate()?;
                vm.instruction_pointer.increment();
            }
            DuplicateTwo => {
                vm.stack.duplicate_two()?;
                vm.instruction_pointer.increment();
            }

            // Arithmetic
            Add => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Int(lhs + rhs));
                    }
                    (Obj(lhs_handle), Obj(rhs_handle)) => {
                        let lhs_object = vm.memory.load(lhs_handle.into())?;
                        let rhs_object = vm.memory.load(rhs_handle.into())?;

                        match (lhs_object.data, rhs_object.data) {
                            (
                                ObjectData::String(lhs_string),
                                ObjectData::String(rhs_string),
                            ) => {
                                let concatenated_string =
                                    format!("{}{}", lhs_string.0, rhs_string.0);
                                let new_object =
                                    Object::new_string(concatenated_string);
                                let new_object_handle = vm
                                    .memory
                                    .store(new_object)?
                                    .cast_to_object_handle();
                                vm.stack.push(Value::Obj(new_object_handle));
                            }
                            _ => {
                                return Err(
                                    MathError::UnsupportedAdditionOperands(
                                        Value::Obj(lhs_handle),
                                        Value::Obj(rhs_handle),
                                    )
                                    .into(),
                                );
                            }
                        }
                    }
                    (x, y) => {
                        return Err(MathError::UnsupportedAdditionOperands(
                            x, y,
                        )
                        .into());
                    }
                }

                vm.instruction_pointer.increment();
            }
            Subtract => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Int(lhs - rhs));
                    }
                    // The usage of the VM should ensure that only integers
                    // are attempted to be subtracted.
                    _ => unreachable!(),
                }

                vm.instruction_pointer.increment();
            }
            Multiply => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Int(lhs * rhs));
                    }
                    // The usage of the VM should ensure that only integers
                    // are attempted to be multiplied.
                    _ => unreachable!(),
                }

                vm.instruction_pointer.increment();
            }
            Divide => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Int(lhs / rhs));
                    }
                    // The usage of the VM should ensure that only integers
                    // are attempted to be divided.
                    _ => unreachable!(),
                }

                vm.instruction_pointer.increment();
            }
            Modulo => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Int(lhs % rhs));
                    }
                    _ => {
                        return Err(
                            MathError::UnsupportedModuloOperands(a, b).into()
                        );
                    }
                }

                vm.instruction_pointer.increment();
            }
            Power => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Int(lhs.pow(rhs as u32)));
                    }
                    _ => {
                        return Err(
                            MathError::UnsupportedPowerOperands(a, b).into()
                        );
                    }
                }

                vm.instruction_pointer.increment();
            }

            // Comparison
            Equals => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Bool(lhs == rhs));
                    }
                    // The usage of the VM should ensure that only integers
                    // are attempted to be compared.
                    _ => unreachable!(),
                }

                vm.instruction_pointer.increment();
            }
            LessThan => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Bool(lhs < rhs));
                    }
                    // The usage of the VM should ensure that only integers
                    // are attempted to be compared.
                    _ => unreachable!(),
                }

                vm.instruction_pointer.increment();
            }
            GreaterThan => {
                use Value::*;

                let b = vm.stack.pop()?;
                let a = vm.stack.pop()?;

                match (a, b) {
                    (Int(lhs), Int(rhs)) => {
                        vm.stack.push(Bool(lhs > rhs));
                    }
                    // The usage of the VM should ensure that only integers
                    // are attempted to be compared.
                    _ => unreachable!(),
                }

                vm.instruction_pointer.increment();
            }
            NotEquals => todo!(),
            LessThanOrEqual => todo!(),
            GreaterThanOrEqual => todo!(),

            // Control flow
            Jump(offset) => {
                // Safety check to ensure that no dummy values are used for
                // jumps.
                if offset == InstructionOffset::DUMMY {
                    return Err(VmError::DummyInstructionOffset);
                }

                vm.instruction_pointer.add_offset(offset);
            }
            JumpIfTrue(offset) => {
                use Value::*;

                let cond = vm.stack.pop()?;
                match cond {
                    Bool(true) => {
                        vm.instruction_pointer.add_offset(offset);
                    }
                    Bool(false) => {
                        vm.instruction_pointer.increment();
                    }
                    // The usage of the VM should ensure that only boolean
                    // values are used for
                    // conditional jumps.
                    _ => unreachable!(),
                }
            }
            JumpIfFalse(offset) => {
                use Value::*;

                let cond = vm.stack.pop()?;
                match cond {
                    Bool(true) => {
                        vm.instruction_pointer.increment();
                    }
                    Bool(false) => {
                        vm.instruction_pointer.add_offset(offset);
                    }
                    // The usage of the VM should ensure that only boolean
                    // values are used for
                    // conditional jumps.
                    _ => unreachable!(),
                }
            }

            // Memory
            Load(addr) => {
                vm.stack.push(Value::Obj(
                    // Cast the memory address to an object handle
                    // The memory address should always be a valid object
                    // handle
                    addr.cast_to_object_handle(),
                ));

                vm.instruction_pointer.increment();
            }
            Store(_addr) => {
                todo!("The STORE instruction is not yet implemented");
            }
            SetLocal(slot) => {
                let value = vm
                    .stack
                    .peek()
                    .ok_or(ValueStackError::StackUnderflow)?
                    .clone();

                vm.stack.set_at(slot.as_usize(), value)?;
                vm.instruction_pointer.increment();
            }
            GetLocal(slot) => {
                let value = vm.stack.get_at(slot.as_usize())?.clone();

                vm.stack.push(value);
                vm.instruction_pointer.increment();
            }

            // Constants
            Constant(addr) => {
                use bytecode::Constant;

                // Clone the constant from the assembly's constant pool and
                // convert it into a value that can be
                // pushed onto the stack.
                let value = match vm
                    .assembly
                    .constants
                    .get(addr)
                    .ok_or(InvalidConstantAddress(addr))?
                    .clone()
                {
                    Constant::Nil => Value::Nil,
                    Constant::Int(i) => Value::Int(i),
                    Constant::Boolean(b) => Value::Bool(b),
                    Constant::Float(_) => {
                        todo!("Float constants are not yet supported")
                    }
                    Constant::String(s) => Value::Obj(
                        // Cast the memory address to an object handle
                        // because we know that the memory address returned
                        // by `store` will always be a valid object handle.
                        vm.memory
                            .store(Object::new_string(s))?
                            .cast_to_object_handle(),
                    ),
                };

                vm.stack.push(value);
                vm.instruction_pointer.increment();
            }

            // Functions
            Call(addr) => {
                let address_to_return_to_when_call_returns =
                    vm.instruction_pointer.as_usize() + 1;

                vm.call_stack
                    .push(address_to_return_to_when_call_returns.into());

                vm.instruction_pointer = addr.into();
            }
            Return => {
                let address_to_return_to =
                    vm.call_stack.pop().ok_or(CallStackUnderflow)?;

                vm.instruction_pointer = address_to_return_to.into();
            }

            // IO
            In => {
                let input = vm.io.read_line()?;

                vm.stack.push(Value::Obj(
                    // Cast the memory address to an object handle
                    // because we know that the memory address returned
                    // by `store` will always be a valid object handle.
                    vm.memory
                        .store(Object::new_string(input))?
                        .cast_to_object_handle(),
                ));

                vm.instruction_pointer.increment();
            }
            Out => {
                let value = vm.stack.pop()?;

                match value {
                    Value::Obj(object_handle) => {
                        let object = vm.memory.load(object_handle.into())?;

                        match object.data {
                            ObjectData::String(str) => {
                                vm.io.write_line(&str.to_io_string())?;
                            }
                            ObjectData::Function(func) => {
                                vm.io.write_line(&func.to_io_string())?;
                            }
                        }
                    }
                    x => {
                        vm.io.write_line(&x.to_io_string())?;
                    }
                }

                vm.instruction_pointer.increment();
            }
            SetGlobal(constant_address) => {
                // That we don’t pop the value until after we add it to the
                // hash table. This ensures the VM can still find the value
                // if a garbage collection is triggered right in the middle
                // of adding it to the hash table.
                let value = vm
                    .stack
                    .peek()
                    .ok_or(ValueStackError::StackUnderflow)?
                    .clone();

                let name = match vm.assembly.constants.get(constant_address) {
                    Some(bytecode::Constant::String(name)) => name,
                    Some(constant) => {
                        return Err(InvalidConstantType(
                            constant_address,
                            "String",
                            constant.clone(),
                        ));
                    }
                    None => {
                        return Err(InvalidConstantAddress(constant_address));
                    }
                };

                vm.global_environment.insert(name.clone(), value);
                vm.stack.pop()?;
                vm.instruction_pointer.increment();
            }
            GetGlobal(constant_address) => {
                let name = match vm
                    .assembly
                    .constants
                    .get(constant_address)
                    .ok_or(InvalidConstantAddress(constant_address))?
                    .clone()
                {
                    bytecode::Constant::String(s) => s,
                    other => {
                        return Err(InvalidConstantType(
                            constant_address,
                            "String",
                            other,
                        ));
                    }
                };

                let value = vm
                    .global_environment
                    .get(&name)
                    .ok_or_else(|| InvalidConstantAddress(constant_address))?
                    .clone();

                vm.stack.push(value);
                vm.instruction_pointer.increment();
            }

            Debug => {
                let ip = vm.instruction_pointer;
                let stack = &vm.stack;
                let call_stack = &vm.call_stack;
                let globals = &vm.global_environment;
                let memory = &vm.memory;

                let mut message = String::new();
                message.push_str(&format!(
                    "Instruction Pointer: {}\n",
                    ip.as_usize()
                ));
                message.push_str(&format!(
                    "Call Stack: {:?}\n",
                    call_stack.clone().dump()
                ));
                message.push_str(&format!(
                    "Value Stack: {:?}\n",
                    stack.clone().dump()
                ));
                message.push_str(&format!(
                    "Memory: {:?}\n",
                    memory.clone().dump()
                ));
                message.push_str(&format!("Globals: {:?}\n", globals.table));
                vm.io.write_line(&message)?;

                vm.instruction_pointer.increment();
            }

            Halt => break Ok(()),
            _ => todo!(),
        }
    }
}
