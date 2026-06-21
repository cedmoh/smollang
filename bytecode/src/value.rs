use crate::MemoryAddress;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Nil,
    Int(i32),
    Boolean(bool),
    Object(ObjectHandle),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ObjectHandle(MemoryAddress);

/// A handle to an object in the heap.
/// This is what gets stored on the stack when an object is created.
impl ObjectHandle {
    pub fn new(address: MemoryAddress) -> Self {
        Self(address)
    }

    pub fn into_memory_address(&self) -> MemoryAddress {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    // TODO: Use when implementing garbage collection
    _marked: bool,
    pub data: ObjectData,
}

impl Object {
    pub fn new(data: ObjectData) -> Self {
        Self {
            _marked: false,
            data,
        }
    }

    pub fn new_string(chars: String) -> Self {
        Self::new(ObjectData::String(ObjectString::new(chars)))
    }
}

#[derive(Debug, Clone)]
pub enum ObjectData {
    String(ObjectString),
}

#[derive(Debug, Clone)]
pub struct ObjectString(pub String);

impl ObjectString {
    pub fn new(chars: String) -> Self {
        Self(chars)
    }
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}i", i),
            Value::Boolean(b) => match b {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            Value::Object(object_handle) => {
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
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
        assert_eq!(Value::Nil.to_string(), "nil");
    }
}
