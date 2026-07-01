use std::collections::HashMap;

use crate::InstructionAddress;

#[derive(Debug, Clone)]
pub struct Labels(HashMap<InstructionAddress, String>);

impl Labels {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_label(&mut self, label: Label) {
        self.0.insert(label.instruction_index, label.name);
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.add_label(label);
        self
    }

    pub fn get_label(
        &self,
        instruction_index: InstructionAddress,
    ) -> Option<Label> {
        self.0
            .get(&instruction_index)
            .map(|name| Label::new(name.clone(), instruction_index))
    }
}

impl From<Vec<Label>> for Labels {
    fn from(labels: Vec<Label>) -> Self {
        Labels(
            labels
                .into_iter()
                .map(|label| (label.instruction_index, label.name))
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Label {
    pub instruction_index: InstructionAddress,
    pub name: String,
}

impl Label {
    pub fn new(name: String, instruction_index: InstructionAddress) -> Self {
        Self {
            name,
            instruction_index,
        }
    }
}
