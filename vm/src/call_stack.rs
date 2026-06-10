#[derive(Debug)]
pub struct CallStack {
    return_addresses: Vec<CallStackAddress>,
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            return_addresses: Vec::new(),
        }
    }

    pub fn push(&mut self, addr: CallStackAddress) {
        self.return_addresses.push(addr);
    }

    pub fn pop(&mut self) -> Option<CallStackAddress> {
        self.return_addresses.pop()
    }

    pub fn last(&self) -> Option<&CallStackAddress> {
        self.return_addresses.last()
    }
}

pub type CallStackAddress = usize;
