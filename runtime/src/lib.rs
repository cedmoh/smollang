mod environment;
mod evaluator;
mod runner;
mod value;

pub use environment::*;
pub use evaluator::*;
pub use runner::*;
pub use value::*;

#[cfg(test)]
mod tests {
    use ast::{
        Dyadic, DyadicOperator, Expression, Expressions, Identifier, IntegerLiteral, Literal,
        Program, VariableDeclarationBuilder,
    };

    use super::*;

    #[test]
    fn does_not_throw() {
        let mut runner = Runner::new();

        runner.run(Program::default());
    }

    #[test]
    fn does_math() {
        let mut runner = Runner::new();

        let x_identifier = Identifier::new("x".to_string());
        let x_value = 5;
        let y_identifier = Identifier::new("y".to_string());
        let y_value = 10;

        let result = runner.run(Program::new(Expressions::new(vec![
            Expression::VariableDeclaration(
                VariableDeclarationBuilder::new(x_identifier.clone())
                    .with_initial_value(Expression::Literal(Literal::Integer(IntegerLiteral::new(
                        x_value,
                    ))))
                    .build(),
            ),
            Expression::VariableDeclaration(
                VariableDeclarationBuilder::new(y_identifier.clone())
                    .with_initial_value(Expression::Literal(Literal::Integer(IntegerLiteral::new(
                        y_value,
                    ))))
                    .build(),
            ),
            Expression::Dyadic(Dyadic::new(
                DyadicOperator::Add,
                Expression::Identifier(x_identifier),
                Expression::Identifier(y_identifier),
            )),
        ])));

        assert_eq!(result, Value::Number(15.));
    }
}
