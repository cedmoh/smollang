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
        let instr = *vm.current_frame().get_next_instruction();

        match instr {
            // Stack
            Push(value) => {
                vm.stack.push(value);
                vm.increment_instruction_pointer();
            }
            Pop => {
                vm.stack.pop()?;
                vm.increment_instruction_pointer();
            }
            Duplicate => {
                vm.stack.duplicate()?;
                vm.increment_instruction_pointer();
            }
            DuplicateTwo => {
                vm.stack.duplicate_two()?;
                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                vm.offset_instruction_pointer(offset);
            }
            JumpIfTrue(offset) => {
                use Value::*;

                let cond = vm.stack.pop()?;
                match cond {
                    Bool(true) => {
                        vm.offset_instruction_pointer(offset);
                    }
                    Bool(false) => {
                        vm.increment_instruction_pointer();
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
                        vm.increment_instruction_pointer();
                    }
                    Bool(false) => {
                        vm.offset_instruction_pointer(offset);
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

                vm.increment_instruction_pointer();
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

                vm.stack.set_at(
                    slot.as_usize() + vm.current_frame().slot_offset,
                    value,
                )?;
                vm.increment_instruction_pointer();
            }
            GetLocal(slot) => {
                let value = vm
                    .stack
                    .get_at(slot.as_usize() + vm.current_frame().slot_offset)?
                    .clone();

                vm.stack.push(value);
                vm.increment_instruction_pointer();
            }

            // Constants
            Constant(addr) => {
                use bytecode::Constant;

                // Clone the constant from the assembly's constant pool and
                // convert it into a value that can be
                // pushed onto the stack.
                let value = match vm
                    .current_frame()
                    .function_object
                    .chunk
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
                vm.increment_instruction_pointer();
            }

            // Functions
            Call(_address) => {
                todo!()
            }
            Return => {
                todo!()
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

                vm.increment_instruction_pointer();
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

                vm.increment_instruction_pointer();
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

                let name = match vm
                    .current_frame()
                    .function_object
                    .chunk
                    .constants
                    .get(constant_address)
                {
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
                vm.increment_instruction_pointer();
            }
            GetGlobal(constant_address) => {
                let name = match vm
                    .current_frame()
                    .function_object
                    .chunk
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
                vm.increment_instruction_pointer();
            }

            Debug => {
                let ip = vm.get_current_instruction_address();
                let stack = &vm.stack;
                let call_stack = &vm.frames;
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

                vm.increment_instruction_pointer();
            }

            Halt => break Ok(()),
            _ => todo!(),
        }
    }
}
