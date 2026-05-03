mod environment;
mod evaluator;
mod function;
mod runner;
mod scope;
mod value;

pub use environment::*;
pub use evaluator::*;
pub use function::*;
pub use runner::*;
pub use scope::*;
pub use value::*;

#[cfg(test)]
mod tests {
    use ast::{
        Dyadic, DyadicOperator, Expression, Expressions, Identifier, IntegerLiteral, Literal,
        Program, VariableDeclarationBuilder,
    };

    use super::*;

    #[test]
    fn should_not_throw() {
        // Arrange
        let mut runner = Runner::new();

        // Act
        runner.run(Program::default());
    }

    #[test]
    fn should_be_able_to_use_global_variables() {
        // Arrange
        let mut runner = Runner::new();

        let x_identifier = "x";
        let x_value = 5.;

        runner
            .environment
            .add_global_variable(x_identifier, Value::Number(x_value))
            .expect("The variable is not defined in the global scope");

        // Act
        // The program is just an identifier expression that references the global variable that we just defined.
        let program = Program::new(Expressions::new(vec![Expression::Identifier(
            Identifier::new(x_identifier.to_string()),
        )]));

        let result = runner.run(program);

        // Assert
        assert_eq!(result, Value::Number(5.));
    }

    #[test]
    fn should_run_the_program_and_return_the_last_expression() {
        // Arrange
        let mut runner = Runner::new();

        let x_identifier = Identifier::new("x".to_string());
        let x_value = 5;
        let y_identifier = Identifier::new("y".to_string());
        let y_value = 10;

        // Act
        // The program is a sequence of expressions that declares two variables and then adds them together.
        let program = Program::new(Expressions::new(vec![
            // Declare a variable `x` with the value of `5`.
            Expression::VariableDeclaration(
                VariableDeclarationBuilder::new(x_identifier.clone())
                    .with_initial_value(Expression::Literal(Literal::Integer(IntegerLiteral::new(
                        x_value,
                    ))))
                    .build(),
            ),
            // Declare a variable `y` with the value of `10`.
            Expression::VariableDeclaration(
                VariableDeclarationBuilder::new(y_identifier.clone())
                    .with_initial_value(Expression::Literal(Literal::Integer(IntegerLiteral::new(
                        y_value,
                    ))))
                    .build(),
            ),
            // Add `x` and `y` together as the last expression of the program, which should be returned as the result of running the program.
            Expression::Dyadic(Dyadic::new(
                DyadicOperator::Add,
                Expression::Identifier(x_identifier),
                Expression::Identifier(y_identifier),
            )),
        ]));
        let result = runner.run(program);

        // Assert
        assert_eq!(result, Value::Number(15.));
    }
}
