use bytecode::{FunctionObject, StringObject, Value};

/// A trait for displaying values to the user.
/// Called by the `OUT` instruction to format values for output.
pub trait ToIoString {
    fn to_io_string(&self) -> String;
}

impl ToIoString for Value {
    fn to_io_string(&self) -> String {
        use Value::*;

        match self {
            Nil => format!("nil"),
            Int(integer) => format!("{}", integer),
            Bool(boolean) => format!("{}", boolean),
            Obj(object_handle) => {
                format!("[Object:{}]", object_handle.into_memory_address())
            }
        }
    }
}

impl ToIoString for String {
    fn to_io_string(&self) -> String {
        self.clone()
    }
}

impl ToIoString for &str {
    fn to_io_string(&self) -> String {
        self.to_string()
    }
}

impl ToIoString for StringObject {
    fn to_io_string(&self) -> String {
        self.0.clone()
    }
}
impl ToIoString for FunctionObject {
    fn to_io_string(&self) -> String {
        todo!("Implement ToIoString for FunctionObject");
    }
}
