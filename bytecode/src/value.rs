use std::fmt::Display;

use crate::Constant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Nil,
    Int(i32),
    Boolean(bool),
    Object(ObjectHandle),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectHandle(pub usize);

#[derive(Debug)]
pub struct Object {
    marked: bool, // Used for garbage collection
    data: ObjectData,
}

#[derive(Debug)]
pub enum ObjectData {
    String(ObjectString),
}

#[derive(Debug)]
pub struct ObjectString {
    pub chars: String,
    pub length: usize,
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<Constant> for Value {
    fn from(value: Constant) -> Self {
        match value {
            Constant::Nil => Value::Nil,
            Constant::Int(int) => Value::Int(int),
            Constant::Boolean(boolean) => Value::Boolean(boolean),
            Constant::Float(_) | Constant::String(_) => {
                todo!("Only nil, integer, and boolean constants are supported")
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}i", i),
            Value::Boolean(b) => match b {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            Value::Object(_) => write!(f, "<object>"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Int(42).to_string(), "42i");
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
        assert_eq!(Value::Nil.to_string(), "nil");
    }

    #[test]
    fn should_convert_supported_constants_to_values() {
        assert!(matches!(Value::from(Constant::Nil), Value::Nil));
        assert!(matches!(Value::from(Constant::Int(42)), Value::Int(42)));
        assert!(matches!(
            Value::from(Constant::Boolean(true)),
            Value::Boolean(true)
        ));
    }
}
