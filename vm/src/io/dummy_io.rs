use thiserror::Error;

use crate::io::Io;

/// A dummy implementation of the `Io` trait for testing purposes.
/// It allows recording output and simulating input without actually performing
/// any real I/O operations.
#[derive(Debug)]
pub struct DummyIo {
    stdin: Vec<String>,
    stdout: Vec<String>,
    stderr: Vec<String>,
}

impl DummyIo {
    /// Creates a new `DummyIo` instance with empty input and output buffers.
    pub fn new() -> Self {
        Self {
            stdin: Vec::new(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }
}

impl Io<DummyIoError> for DummyIo {
    fn read_line(&mut self) -> Result<String, DummyIoError> {
        self.stdin.pop().ok_or(DummyIoError::NoInput)
    }

    fn write_line(&mut self, line: &str) {
        self.stdout.push(line.to_string());
    }

    fn write_error_line(&mut self, line: &str) {
        self.stderr.push(line.to_string());
    }
}

#[derive(Debug, Error)]
pub enum DummyIoError {
    /// Error indicating that there is no input available in the stdin buffer.
    #[error("No input available in stdin")]
    NoInput,
}
