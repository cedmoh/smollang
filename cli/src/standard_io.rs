use std::io::{Error, stdin};
use vm::Io;

pub struct StandardIo;

impl StandardIo {
    pub fn new() -> Self {
        Self
    }
}

impl Io<Error> for StandardIo {
    fn read_line(&mut self) -> Result<String, Error> {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        Ok(input.trim_end().to_string())
    }

    fn write_line(&mut self, line: &str) {
        println!("{}", line);
    }

    fn write_error_line(&mut self, line: &str) {
        eprintln!("{}", line);
    }
}
