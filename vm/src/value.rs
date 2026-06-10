use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i32),
    Boolean(bool),
    Addr(usize),
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

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::Addr(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}_i32", i),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Addr(a) => write!(f, "{:08x}", a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Int(42).to_string(), "42_i32");
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
        assert_eq!(Value::Addr(0x00001234).to_string(), "00001234");
    }
}
