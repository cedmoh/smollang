use std::fmt::Display;

/// A relative offset from a program address, used for jump instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionOffset(usize);

impl InstructionOffset {
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for InstructionOffset {
    fn from(offset: usize) -> Self {
        Self(offset)
    }
}

impl Display for InstructionOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = match self.0.cmp(&0) {
            std::cmp::Ordering::Less => "-",
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => "+",
        };

        write!(f, "{}{:0>4}", sign, self.0)
    }
}
