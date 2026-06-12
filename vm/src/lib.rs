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
    fn should_push_items_in_the_right_order() {
        let instructions = bytecode!(
            PUSH 1
            PUSH 2
            PUSH 3
            HALT
        );

        let mut vm = Vm::new();
        vm.load_program(instructions).run().unwrap();

        assert_eq!(vm.stack[0], Value::Int(1));
        assert_eq!(vm.stack[1], Value::Int(2));
        assert_eq!(vm.stack[2], Value::Int(3));
        assert_eq!(vm.stack.len(), 3);
    }

    #[test]
    fn should_pop_items_in_the_right_order() {
        let instructions = bytecode!(
            PUSH 1
            PUSH 2
            PUSH 3
            POP
            POP
            HALT
        );

        let mut vm = Vm::new();
        vm.load_program(instructions).run().unwrap();

        assert_eq!(vm.stack[0], Value::Int(1));
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn should_duplicate_top_value() {
        let instructions = bytecode!(
            PUSH 1
            DUP
            HALT
        );

        let mut vm = Vm::new();
        vm.load_program(instructions).run().unwrap();

        assert_eq!(vm.stack[0], Value::Int(1));
        assert_eq!(vm.stack[1], Value::Int(1));
        assert_eq!(vm.stack.len(), 2);
    }

    #[test]
    fn should_duplicate_top_two_values_in_order() {
        let instructions = bytecode!(
            PUSH 1
            PUSH 2
            DUP2
            HALT
        );

        let mut vm = Vm::new();
        vm.load_program(instructions).run().unwrap();

        assert_eq!(vm.stack[0], Value::Int(1));
        assert_eq!(vm.stack[1], Value::Int(2));
        assert_eq!(vm.stack[2], Value::Int(1));
        assert_eq!(vm.stack[3], Value::Int(2));
        assert_eq!(vm.stack.len(), 4);
    }

    #[test]
    fn should_consume_and_add_two_numbers_together_and_push_the_result() {
        let instructions = bytecode!(
            PUSH 2
            PUSH 3
            ADD
            HALT
        );

        let mut vm = Vm::new();
        vm.load_program(instructions).run().unwrap();

        assert_eq!(vm.stack[0], Value::Int(5));
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn should_jump_unconditionally() {
        let instructions = bytecode!(
            JUMP 2
            HALT // This HALT should be skipped
            PUSH 42
            HALT
        );

        let instructions_len = instructions.len();

        let mut vm = Vm::new();
        vm.load_program(instructions).run().unwrap();

        assert_eq!(vm.instruction_pointer.as_usize(), instructions_len - 1);
        assert_eq!(vm.stack[0], Value::Int(42));
    }
}
