mod dummy_io;

pub use dummy_io::*;

pub trait Io<E> {
    fn read_line(&mut self) -> Result<String, E>;
    fn write_line(&mut self, line: &str);
    fn drain_stdout(&mut self) -> Result<String, E>;
}
