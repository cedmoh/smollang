use crate::{EvaluationResult, Value};
use ast::Literal;

/// Evaluate a literal expression by converting the literal to a value.
pub fn evaluate_literal(literal: Literal) -> EvaluationResult {
    use ast::Literal::*;

    let value = match literal {
        Nil => Value::Nil,
        Boolean(boolean) => Value::Boolean(boolean.value),
        String(string) => Value::String(string.value),
        Template(template) => Value::String(template.value),
        Integer(int) => Value::Number(int.value as f64),
        Decimal(decimal_literal) => Value::Number(decimal_literal.value),
        Hexadecimal(hexadecimal_literal) => {
            Value::Number(hexadecimal_literal.value as f64)
        }
        Binary(binary_literal) => Value::Number(binary_literal.value as f64),
        Octal(octal_literal) => Value::Number(octal_literal.value as f64),
        Array(_array_literal) => todo!(),
        Object(_object_literal) => todo!(),
    };

    EvaluationResult::Value(value)
}
