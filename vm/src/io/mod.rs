mod dummy_io;
mod io_display;

pub use dummy_io::*;
pub use io_display::*;
use thiserror::Error;

pub trait Io {
    fn read_line(&mut self) -> Result<String, IoError>;
    fn write_line(&mut self, line: &str) -> Result<(), IoError>;
    fn drain_stdout(&mut self) -> Result<String, IoError>;
}

/// An error that can occur during I/O operations.
#[derive(Debug, Error)]
pub enum IoError {
    #[error("Read error: {0}")]
    ReadError(Box<dyn std::error::Error + Send + Sync>),

    #[error("Write error: {0}")]
    WriteError(Box<dyn std::error::Error + Send + Sync>),

    #[error("Drain error: {0}")]
    DrainError(Box<dyn std::error::Error + Send + Sync>),
}
