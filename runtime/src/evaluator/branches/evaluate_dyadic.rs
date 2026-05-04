use crate::{EvaluationResult, Evaluator, Scope, Value};
use EvaluationResult::Throw;
use ast::{Dyadic, DyadicOperator};
use std::ops::{Add, Div, Mul, Rem, Sub};

/// Function to evaluate a dyadic expression.
pub fn evaluate_dyadic(
    dyadic: Dyadic,
    evaluator: &Evaluator,
    scope: &mut Scope,
) -> EvaluationResult {
    // evaluate the left expression
    let left_value = match evaluator.evaluate_expression(*dyadic.left, scope) {
        EvaluationResult::Value(value) => value,
        EvaluationResult::Throw(err) => return Throw(err),
        EvaluationResult::Return(_) => {
            return Throw(Value::String(
                "Cannot return directly from a dyadic expression.".to_string(),
            ));
        }
    };

    let right_value = match evaluator.evaluate_expression(*dyadic.right, scope) {
        EvaluationResult::Value(value) => value,
        EvaluationResult::Throw(err) => return Throw(err),
        EvaluationResult::Return(_) => {
            return Throw(Value::String(
                "Cannot return directly from a dyadic expression.".to_string(),
            ));
        }
    };

    let operation_result = match dyadic.operator {
        DyadicOperator::Add => left_value.add(right_value),
        DyadicOperator::Subtract => left_value.sub(right_value),
        DyadicOperator::Multiply => left_value.mul(right_value),
        DyadicOperator::Divide => left_value.div(right_value),
        DyadicOperator::Modulo => left_value.rem(right_value),
        DyadicOperator::Power => left_value.pow(right_value),
        DyadicOperator::Equal => Ok(Value::Boolean(left_value.eq(&right_value))),
        DyadicOperator::NotEqual => Ok(Value::Boolean(left_value.ne(&right_value))),
        DyadicOperator::LessThan => Ok(Value::Boolean(left_value.lt(&right_value))),
        DyadicOperator::GreaterThan => Ok(Value::Boolean(left_value.gt(&right_value))),
        DyadicOperator::LessThanOrEqual => Ok(Value::Boolean(left_value.le(&right_value))),
        DyadicOperator::GreaterThanOrEqual => Ok(Value::Boolean(left_value.ge(&right_value))),
        DyadicOperator::And => left_value.and(right_value),
        DyadicOperator::Or => left_value.or(right_value),
        DyadicOperator::RangeInclusive => todo!("When arrays are implemented"),
        DyadicOperator::Range => todo!("When arrays are implemented"),
    };

    match operation_result {
        Ok(value) => EvaluationResult::Value(value),
        Err(err) => Throw(Value::String(err)),
    }
}
