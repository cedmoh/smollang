use std::io::stdin;
use vm::{Io, IoError};

/// A struct that implements the `Io` trait for standard input and output.
pub struct StandardIo;

impl StandardIo {
    pub fn new() -> Self {
        Self
    }
}

impl Io for StandardIo {
    fn read_line(&mut self) -> Result<String, IoError> {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .map_err(|e| IoError::ReadError(Box::new(e)))?;
        Ok(input.trim_end().to_string())
    }

    fn write_line(&mut self, line: &str) -> Result<(), IoError> {
        println!("{}", line);
        Ok(())
    }

    fn drain_stdout(&mut self) -> Result<String, IoError> {
        todo!("Draining stdout is not supported for StandardIo")
    }
}
