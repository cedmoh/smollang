mod object;

pub use object::*;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Nil,
    Int(i32),
    Bool(bool),
    Obj(ObjectHandle),
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}i", i),
            Value::Bool(b) => match b {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            Value::Obj(object_handle) => {
                write!(f, "[Object:{}]", object_handle.into_memory_address())
            }
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
        assert_eq!(Value::Bool(true).to_string(), "true");
        assert_eq!(Value::Bool(false).to_string(), "false");
        assert_eq!(Value::Nil.to_string(), "nil");
    }
}
