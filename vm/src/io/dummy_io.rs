use std::collections::VecDeque;

use thiserror::Error;

use crate::io::{Io, ToIoString};

/// A dummy implementation of the `Io` trait for testing purposes.
/// It allows recording output and simulating input without actually performing
/// any real I/O operations.
#[derive(Debug)]
pub struct DummyIo {
    pub stdin: VecDeque<String>,
    pub stdout: VecDeque<String>,
}

impl DummyIo {
    /// Creates a new `DummyIo` instance with empty input and output buffers.
    pub fn new() -> Self {
        Self {
            stdin: VecDeque::new(),
            stdout: VecDeque::new(),
        }
    }
}

impl Io<DummyIoError> for DummyIo {
    fn read_line(&mut self) -> Result<String, DummyIoError> {
        self.stdin.pop_front().ok_or(DummyIoError::NoInput)
    }

    fn write_line(&mut self, line: &str) {
        self.stdout.push_back(line.to_io_string());
    }

    fn drain_stdout(&mut self) -> Result<String, DummyIoError> {
        Ok(self.stdout.drain(..).collect::<Vec<String>>().join("\n"))
    }
}

#[derive(Debug, Error)]
pub enum DummyIoError {
    /// Error indicating that there is no input available in the stdin buffer.
    #[error("No input available in stdin")]
    NoInput,

    /// Error indicating that there was a failure to format the output for writing.
    #[error("Failed to format output for writing: {0}")]
    FormatError(#[from] std::fmt::Error),
}
