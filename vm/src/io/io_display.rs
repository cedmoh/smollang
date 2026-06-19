use bytecode::Value;

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
            Boolean(boolean) => format!("{}", boolean),
            Object(object_handle) => {
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
