mod standard_io;

pub use standard_io::StandardIo;

pub trait Io {
    /// Reads a value from the input. Returns `None` if there is no more input
    /// available.
    fn read(&mut self) -> String;

    /// Writes a value to the output.
    fn write(&mut self, value: &str);
}
