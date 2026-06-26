mod call_stack;
mod global_environment;
mod io;
mod memory;
mod value_stack;
mod vm;

pub use io::{Io, IoError};
pub use vm::Vm;

#[cfg(test)]
mod tests {
    use super::*;
    use bytecode::{
        Assembly, Constant, Instruction, MemoryAddress, Object, ObjectData,
        ObjectHandle, Value, bytecode,
    };

    #[test]
    fn should_parse_negative_jump_offset_in_bytecode_macro() {
        let instructions = bytecode!(
            JUMP -2
            HALT
        );

        assert_eq!(
            instructions,
            vec![Instruction::Jump((-2isize).into()), Instruction::Halt]
        );
    }

    #[test]
    fn should_push_items_in_the_right_order() {
        let instructions = bytecode!(
            PUSH 1
            PUSH 2
            PUSH 3
            HALT
        );

        let mut vm = Vm::new();
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

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
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

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
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

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
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

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
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

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
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

        assert_eq!(vm.instruction_pointer.as_usize(), instructions_len - 1);
        assert_eq!(vm.stack[0], Value::Int(42));
    }

    #[test]
    fn should_push_constant_pool_values_when_const_instruction_is_used() {
        use Constant::*;

        let instructions = bytecode!(
            CONST 0
            CONST 1
            CONST 2
            HALT
        );

        let constants = vec![Nil, Int(42), Boolean(true)];

        let mut vm = Vm::new();
        vm.load_assembly(
            Assembly::builder()
                .instructions(instructions)
                .constants(constants)
                .build(),
        )
        .run()
        .unwrap();

        assert_eq!(vm.stack[0], Value::Nil);
        assert_eq!(vm.stack[1], Value::Int(42));
        assert_eq!(vm.stack[2], Value::Boolean(true));
        assert_eq!(vm.stack.len(), 3);
    }

    #[test]
    fn should_print_strings() {
        use Constant::*;

        // Arrange
        let instructions = bytecode!(
            CONST 0
            OUT
            HALT
        );

        let string = "Hello, world!".to_string();

        let constants = vec![String(string.clone())];
        let assembly = Assembly::builder()
            .instructions(instructions)
            .constants(constants)
            .build();

        let mut vm = Vm::new();

        // Act
        vm.load_assembly(assembly).run().unwrap();

        // Assert
        assert_eq!(vm.io.drain_stdout().unwrap(), string);
    }

    #[test]
    fn should_read_input() {
        // Arrange
        let instructions = bytecode!(
            IN
            HALT
        );

        let input = "Hello, world!".to_string();

        let mut vm = Vm::new();
        vm.io.stdin.push_back(input.clone());

        // Act
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

        // Assert
        assert_eq!(
            vm.stack.pop().unwrap(),
            Value::Object(ObjectHandle::new(MemoryAddress::new(0)))
        );
        std::assert_matches!(
            vm.memory.load(MemoryAddress::new(0)).unwrap().data,
            ObjectData::String(s) if s.0 ==     input,
        );
    }

    #[test]
    fn should_get_local_and_push_its_value() {
        let instructions = bytecode!(
            PUSH 10
            PUSH 20
            GETLC 0
            HALT
        );

        let mut vm = Vm::new();
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

        assert_eq!(vm.stack[0], Value::Int(10));
        assert_eq!(vm.stack[1], Value::Int(20));
        assert_eq!(vm.stack[2], Value::Int(10));
    }

    #[test]
    fn should_set_local_without_popping_stack_top() {
        let instructions = bytecode!(
            PUSH 1
            PUSH 2
            PUSH 9
            SETLC 0
            HALT
        );

        let mut vm = Vm::new();
        vm.load_assembly(
            Assembly::builder().instructions(instructions).build(),
        )
        .run()
        .unwrap();

        assert_eq!(vm.stack[0], Value::Int(9));
        assert_eq!(vm.stack[1], Value::Int(2));
        assert_eq!(vm.stack[2], Value::Int(9));
    }
}
