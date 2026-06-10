mod bytecode_macro;
mod call_stack;
mod instruction;
mod io;
mod memory;
mod program;
mod value;
mod value_stack;
mod vm;

pub use instruction::Instruction;
pub use value::Value;
pub use vm::Vm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_two_numbers() {
        // This program demonstrates basic arithmetic. It pushes the values 2
        // and 3 onto the stack, adds them together, and prints the
        // result (5).
        let instructions = bytecode!(
            PUSH 2
            PUSH 3
            ADD
            PRINT
            HALT
        );

        Vm::new().load_program(instructions).run().unwrap();
    }

    #[test]
    fn should_jump_unconditionally() {
        // This program demonstrates an unconditional jump. It pushes the
        // value 42 onto the stack and then jumps over to the instruction
        // that prints it.
        let instructions = bytecode!(
            JUMP 2
            HALT
            PUSH 42
            PRINT
            HALT
        );

        Vm::new().load_program(instructions).run().unwrap();
    }
}
