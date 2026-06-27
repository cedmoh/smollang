use bytecode::InstructionAddress;

#[derive(Debug, Clone)]
pub struct CallStack {
    return_addresses: Vec<InstructionAddress>,
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            return_addresses: Vec::new(),
        }
    }

    pub fn push(&mut self, addr: InstructionAddress) {
        self.return_addresses.push(addr);
    }

    pub fn pop(&mut self) -> Option<InstructionAddress> {
        self.return_addresses.pop()
    }

    pub fn dump(self) -> Vec<InstructionAddress> {
        self.return_addresses.clone()
    }
}
