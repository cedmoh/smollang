use std::fmt::Display;

use crate::{Assembly, MemoryAddress};

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
        Self::new(ObjectData::String(StringObject::new(chars)))
    }
}

#[derive(Debug, Clone)]
pub enum ObjectData {
    /// A string object
    String(StringObject),

    /// A top level function object
    Function(FunctionObject),
    // TODO: Closure
}

#[derive(Debug, Clone)]
pub struct StringObject(pub String);

impl StringObject {
    pub fn new(chars: String) -> Self {
        Self(chars)
    }
}

impl From<String> for StringObject {
    fn from(chars: String) -> Self {
        Self::new(chars)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionObject {
    /// The name of the function, if it has one.
    pub name: StringObject,

    /// The number of parameters the function takes.
    pub arity: usize,

    /// The bytecode entry point of the function.
    pub chunk: Assembly,
}

impl FunctionObject {
    pub fn new(name: StringObject, arity: usize, chunk: Assembly) -> Self {
        Self { name, arity, chunk }
    }
}

impl Display for FunctionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name.0)
    }
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    /// A top-level function
    TopLevel,

    /// A regular function
    Function,
}
