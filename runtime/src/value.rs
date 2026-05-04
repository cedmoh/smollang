use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

/// The result of evaluating an operation on two values.
pub type ValueOperationResult = Result<Value, String>;

impl Add for Value {
    type Output = ValueOperationResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left_num), Value::Number(right_num)) => {
                Ok(Value::Number(left_num + right_num))
            }
            (Value::String(left_str), Value::String(right_str)) => {
                Ok(Value::String(left_str + &right_str))
            }
            (left, right) => Err(format!("Cannot add values '{:?}' and '{:?}'.", left, right)),
        }
    }
}

impl Sub for Value {
    type Output = ValueOperationResult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left_num), Value::Number(right_num)) => {
                Ok(Value::Number(left_num - right_num))
            }
            (left, right) => Err(format!(
                "Cannot subtract values '{:?}' and '{:?}'.",
                left, right
            )),
        }
    }
}

impl Mul for Value {
    type Output = ValueOperationResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left_num), Value::Number(right_num)) => {
                Ok(Value::Number(left_num * right_num))
            }
            (Value::String(string), Value::Number(number))
            | (Value::Number(number), Value::String(string)) => {
                Ok(Value::String(string.repeat(number as usize)))
            }
            (left, right) => Err(format!(
                "Cannot multiply values '{:?}' and '{:?}'.",
                left, right
            )),
        }
    }
}

impl Div for Value {
    type Output = ValueOperationResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left_num), Value::Number(right_num)) => {
                if right_num == 0.0 {
                    Err("Division by zero.".to_string())
                } else {
                    Ok(Value::Number(left_num / right_num))
                }
            }
            (left, right) => Err(format!(
                "Cannot divide values '{:?}' and '{:?}'.",
                left, right
            )),
        }
    }
}

impl Rem for Value {
    type Output = ValueOperationResult;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left_num), Value::Number(right_num)) => {
                if right_num == 0.0 {
                    Err("Modulo by zero.".to_string())
                } else {
                    Ok(Value::Number(left_num % right_num))
                }
            }
            (left, right) => Err(format!(
                "Cannot take modulo of values '{:?}' and '{:?}'.",
                left, right
            )),
        }
    }
}

impl Value {
    pub fn pow(self, rhs: Self) -> ValueOperationResult {
        match (self, rhs) {
            (Value::Number(left_num), Value::Number(right_num)) => {
                Ok(Value::Number(left_num.powf(right_num)))
            }
            (left, right) => Err(format!(
                "Cannot exponentiate values '{:?}' and '{:?}'.",
                left, right
            )),
        }
    }

    pub fn and(self, rhs: Self) -> ValueOperationResult {
        match (self, rhs) {
            (Value::Boolean(left_bool), Value::Boolean(right_bool)) => {
                Ok(Value::Boolean(left_bool && right_bool))
            }
            (left, right) => Err(format!(
                "Cannot perform logical AND on values '{:?}' and '{:?}'.",
                left, right
            )),
        }
    }

    pub fn or(self, rhs: Self) -> ValueOperationResult {
        match (self, rhs) {
            (Value::Boolean(left_bool), Value::Boolean(right_bool)) => {
                Ok(Value::Boolean(left_bool || right_bool))
            }
            (left, right) => Err(format!(
                "Cannot perform logical OR on values '{:?}' and '{:?}'.",
                left, right
            )),
        }
    }
}
