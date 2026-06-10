use crate::io::Io;

pub struct StandardIo;

impl StandardIo {
    pub fn new() -> Self {
        Self
    }
}

impl Io for StandardIo {
    fn write(&mut self, output: &str) {
        print!("{}", output);
    }

    fn read(&mut self) -> String {
        todo!("Implement standard input reading.")
    }
}
