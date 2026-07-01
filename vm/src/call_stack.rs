use bytecode::{
    Constant, ConstantAddress, FunctionObject, Instruction, InstructionAddress,
};

#[derive(Debug, Clone)]
pub struct CallStack(Vec<CallFrame>);

impl CallStack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn new_with_frame(self, frame: CallFrame) -> Self {
        Self(vec![frame])
    }

    pub fn push(&mut self, frame: CallFrame) {
        self.0.push(frame);
    }

    pub fn pop(&mut self) -> Option<CallFrame> {
        self.0.pop()
    }

    pub fn dump(self) -> Vec<CallFrame> {
        self.0.clone()
    }

    pub fn last(&self) -> Option<&CallFrame> {
        self.0.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut CallFrame> {
        self.0.last_mut()
    }
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function_object: Box<FunctionObject>,

    pub slot_offset: usize,

    pub instruction_pointer: InstructionAddress,
}

impl CallFrame {
    pub fn new(
        function_object: Box<FunctionObject>,
        slot_offset: usize,
        instruction_pointer: InstructionAddress,
    ) -> Self {
        Self {
            function_object,
            slot_offset,
            instruction_pointer,
        }
    }

    pub fn get_next_instruction(&self) -> &Instruction {
        &self.function_object.chunk.instructions[self.instruction_pointer]
    }

    pub fn get_next_instruction_and_advance(&mut self) -> Instruction {
        let instruction = self.get_next_instruction().clone();
        self.instruction_pointer.increment();
        instruction
    }

    pub fn get_constant(&self, address: ConstantAddress) -> &Constant {
        &self.function_object.chunk.constants[address]
    }
}
