mod compiler;
mod visitors;

use visitors::AstToInstructionVisitor;

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

        let program = &Program::builder()
            .add_expression(IntegerLiteral::new(integer).into())
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
}
