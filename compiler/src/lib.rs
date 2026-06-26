mod compiler;
mod globals;
mod visitors;
mod locals;

pub use compiler::Compiler;

#[cfg(test)]
mod tests {
    use super::Compiler;
    use ast::*;
    use bytecode::{Assembly, Constant, Instruction, Value, bytecode};

    #[test]
    fn should_compile_integer_literal() {
        // Arrange
        let integer = 42;

        let program = Program::builder()
            // 42
            .with_expression(IntegerLiteral::synthetic(integer))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).unwrap().instructions.into();

        // Assert
        assert_eq!(
            instructions,
            bytecode!(
                PUSH integer
                HALT
            )
        );
    }

    #[test]
    fn should_compile_boolean_literal() {
        // Arrange
        let boolean = true;

        let program = Program::builder()
            // true
            .with_expression(BooleanLiteral::synthetic(boolean))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).unwrap().instructions.into();

        assert_eq!(
            instructions,
            bytecode!(
                PUSH boolean
                HALT
            )
        );
    }

    #[test]
    fn should_compile_nil_literal() {
        // Arrange
        let program = Program::builder()
            // nil
            .with_expression(Literal::Nil)
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).unwrap().instructions.into();

        assert_eq!(
            instructions,
            bytecode!(
                PUSH nil
                HALT
            )
        );
    }

    #[test]
    fn should_compile_add_integer_operation() {
        // Arrange
        let left = 1;
        let right = 2;

        let program = Program::builder()
            // 1 + 2
            .with_expression(Dyadic::synthetic(
                DyadicOperator::Add,
                IntegerLiteral::synthetic(left),
                IntegerLiteral::synthetic(right),
            ))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).unwrap().instructions.into();

        // Assert
        assert_eq!(
            instructions,
            bytecode!(
                PUSH left
                PUSH right
                ADD
                HALT
            )
        );
    }

    #[test]
    fn should_compile_add_string_operation() {
        // Arrange
        let left = "Hello, ".to_string();
        let right = "world!".to_string();

        let program = Program::builder()
            // "Hello, " + "world!"
            .with_expression(Dyadic::synthetic(
                DyadicOperator::Add,
                StringLiteral::synthetic(left.clone()),
                StringLiteral::synthetic(right.clone()),
            ))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let Assembly {
            instructions,
            constants,
        } = compiler.compile(program).unwrap();

        let instructions: Vec<Instruction> = instructions.into();
        let constants: Vec<Constant> = constants.into();

        // Assert
        assert_eq!(
            instructions,
            bytecode!(
                CONST 0
                CONST 1
                ADD
                HALT
            )
        );

        assert_eq!(
            constants,
            vec![Constant::String(left), Constant::String(right),]
        );
    }

    #[test]
    fn should_handle_variables_with_simple_values() {
        // Arrange
        let identifier_name = "x".to_string();
        let initial_value = 42;

        let identifier = Identifier::synthetic(identifier_name.clone());

        let program = Program::builder()
            // x val 42
            .with_expression(
                VariableDeclaration::builder(identifier.clone())
                    .with_initial_value(IntegerLiteral::synthetic(
                        initial_value,
                    ))
                    .build(),
            )
            // x + x
            .with_expression(Dyadic::synthetic(
                DyadicOperator::Add,
                identifier.clone(),
                identifier,
            ))
            .build();

        let Assembly {
            instructions,
            constants,
        } = Compiler::new().compile(program).unwrap();

        // Act
        let instructions: Vec<Instruction> = instructions.into();
        let constants: Vec<Constant> = constants.into();

        // Assert
        let initial_value = Value::Int(initial_value);

        assert_eq!(
            instructions,
            bytecode!(
                PUSH initial_value
                SETGB 0 // the zero-th constant is the variable's name
                GETGB 0
                GETGB 0
                ADD
                HALT
            )
        );
        assert_eq!(constants, vec![Constant::String(identifier_name),]);
    }

    #[test]
    fn should_handle_loops() {
        // Arrange
        let message = "hello world".to_string();


        let r#loop =
            // loop
            Loop::synthetic(
                // print
                FunctionCallBuilder::new(Identifier::synthetic(
                    "print".to_string(),
                ))
                // 'hello world'
                .with_argument(
                    StringLiteral::synthetic(message.clone()),
                )
                .build(),
            );

        let program = Program::builder()
            .with_expression(r#loop)
            .build();

        let Assembly {
            instructions,
            constants,
        } = Compiler::new().compile(program).unwrap();

        // Act
        let instructions: Vec<Instruction> = instructions.into();
        let constants: Vec<Constant> = constants.into();

        // Assert
        assert_eq!(
            instructions,
            bytecode!(
                CONST 0
                OUT
                JUMP -2
                HALT
            )
        );
        assert_eq!(constants, vec![Constant::String(message),]);
    }

    #[test]
    fn should_compile_local_variable_access_inside_block() {
        // Arrange
        let local_name = "x".to_string();
        let initial_value = 42;

        let identifier = Identifier::synthetic(local_name);
        let block = 
            // (
            BlockBuilder::new()
                // x val 42
                .with_expression(
                    VariableDeclarationBuilder::new(identifier.clone())
                        .with_initial_value(IntegerLiteral::synthetic(
                            initial_value,
                        ))
                        .build(),
                )
                // x
                .with_expression(identifier)
            // )
                .build();

        let program = Program::builder().with_expression(block).build();

        // Act
        let instructions: Vec<Instruction> = Compiler::new()
            .compile(program)
            .unwrap()
            .instructions
            .into();

        // Assert
        assert_eq!(
            instructions,
            bytecode!(
                PUSH initial_value
                GETLC 0
                POP
                HALT
            )
        );
    }

    #[test]
    fn should_compile_assignment_to_local_variable_inside_block() {
        // Arrange
        let local_name = "x".to_string();
        let initial_value = 1;
        let new_value = 9;
        let identifier = Identifier::synthetic(local_name);

        let block = 
        // (
        BlockBuilder::new()
            // x val 1
            .with_expression(
                VariableDeclaration::builder(identifier.clone())
                    .with_initial_value(IntegerLiteral::synthetic(
                        initial_value,
                    ))
                    .build(),
            )
            // x = 9
            .with_expression(Assignment::synthetic(
                identifier.clone(),
                IntegerLiteral::synthetic(new_value),
            ))
            // )
            .build();

        let program = Program::builder().with_expression(block).build();

        // Act
        let instructions: Vec<Instruction> = Compiler::new()
            .compile(program)
            .unwrap()
            .instructions
            .into();

        // Assert
        assert_eq!(
            instructions,
            bytecode!(
                PUSH initial_value
                PUSH new_value
                SETLC 0
                POP
                HALT
            )
        );
    }
}
