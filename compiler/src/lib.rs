mod compiler;
mod visitors;

pub use compiler::Compiler;

#[cfg(test)]
mod tests {
    use super::Compiler;
    use ast::Program;
    use ast::*;
    use bytecode::{Constant, Instruction, Value, bytecode};

    #[test]
    fn should_compile_integer_literal() {
        // Arrange
        let integer = 42;

        let program = Program::builder()
            .with_expression(IntegerLiteral::new(integer))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).instructions.into();

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
            .with_expression(BooleanLiteral::new(boolean))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).instructions.into();

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
        let program = Program::builder().with_expression(Literal::Nil).build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).instructions.into();

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
            .with_expression(Dyadic::new(
                DyadicOperator::Add,
                IntegerLiteral::new(left),
                IntegerLiteral::new(right),
            ))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).instructions.into();

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
            .with_expression(Dyadic::new(
                DyadicOperator::Add,
                StringLiteral::new(left.clone()),
                StringLiteral::new(right.clone()),
            ))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let assembly = compiler.compile(program);
        let instructions: Vec<Instruction> = assembly.instructions.into();
        let constants: Vec<Constant> = assembly.constants.into();

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

        let identifier = Identifier::new(identifier_name.clone());

        let program = Program::builder()
            .with_expression(
                VariableDeclaration::builder(identifier.clone())
                    .with_initial_value(IntegerLiteral::new(initial_value))
                    .build(),
            )
            .with_expression(Dyadic::new(
                DyadicOperator::Add,
                identifier.clone(),
                identifier,
            ))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions: Vec<Instruction> =
            compiler.compile(program).instructions.into();

        // Assert
        let initial_value = Value::Int(initial_value);

        assert_eq!(
            instructions,
            bytecode!(
                PUSH initial_value
                PUSH initial_value
                ADD
                HALT
            )
        );
    }
}
