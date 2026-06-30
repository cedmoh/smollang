use crate::MemoryAddress;

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
    pub name: Option<StringObject>,

    /// The number of parameters the function takes.
    pub arity: usize,

    /// The entry point of the function in the bytecode.
    pub entry: MemoryAddress,
}

impl FunctionObject {
    pub fn new(
        name: Option<StringObject>,
        arity: usize,
        entry: MemoryAddress,
    ) -> Self {
        Self { name, arity, entry }
    }

    pub fn named(
        name: StringObject,
        arity: usize,
        entry: MemoryAddress,
    ) -> Self {
        Self::new(Some(name), arity, entry)
    }

    pub fn anonymous(arity: usize, entry: MemoryAddress) -> Self {
        Self::new(None, arity, entry)
    }
}
