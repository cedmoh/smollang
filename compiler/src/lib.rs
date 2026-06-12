mod compiler;
mod visitors;

pub use compiler::Compiler;

#[cfg(test)]
mod tests {
    use super::Compiler;
    use ast::Program;
    use ast::*;
    use bytecode::bytecode;

    #[test]
    fn should_compile_integer_literal() {
        // Arrange
        let integer = 42;

        let program = Program::builder()
            .with_expression(IntegerLiteral::new(integer))
            .build();

        let mut compiler = Compiler::new();

        // Act
        let instructions = compiler.compile(program);

        assert_eq!(
            instructions,
            bytecode!(
                PUSH integer
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
        let instructions = compiler.compile(program);

        assert_eq!(
            instructions,
            bytecode!(
                PUSH boolean
            )
        );
    }

    #[test]
    fn should_compile_nil_literal() {
        // Arrange
        let program = Program::builder().with_expression(Literal::Nil).build();

        let mut compiler = Compiler::new();

        // Act
        let instructions = compiler.compile(program);

        assert_eq!(
            instructions,
            bytecode!(
                PUSH nil
            )
        );
    }

    #[test]
    fn should_compile_add_operation() {
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
        let instructions = compiler.compile(program);

        assert_eq!(
            instructions,
            bytecode!(
                PUSH left
                PUSH right
                ADD
            )
        );
    }
}
