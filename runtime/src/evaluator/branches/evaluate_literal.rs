use crate::{EvaluationResult, Value};
use ast::Literal::{self, Binary, Boolean, Decimal, Hexadecimal, Integer, Nil, Octal, String};

/// Evaluate a literal expression by converting the literal to a value.
pub fn evaluate_literal(literal: Literal) -> EvaluationResult {
    let value = match literal {
        Nil => Value::Nil,
        Boolean(boolean) => Value::Boolean(boolean.value),
        String(string) => Value::String(string.value),
        Integer(int) => Value::Number(int.value as f64),
        Decimal(decimal_literal) => Value::Number(decimal_literal.value),
        Hexadecimal(hexadecimal_literal) => Value::Number(hexadecimal_literal.value as f64),
        Binary(binary_literal) => Value::Number(binary_literal.value as f64),
        Octal(octal_literal) => Value::Number(octal_literal.value as f64),
    };

    EvaluationResult::Value(value)
}
