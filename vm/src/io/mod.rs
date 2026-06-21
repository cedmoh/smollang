mod dummy_io;
mod io_display;

pub use dummy_io::*;
pub use io_display::*;

pub trait Io<E> {
    fn read_line(&mut self) -> Result<String, E>;
    fn write_line(&mut self, line: &str) -> ();
    fn drain_stdout(&mut self) -> Result<String, E>;
}
