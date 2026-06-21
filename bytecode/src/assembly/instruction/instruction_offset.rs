use std::fmt::Display;

/// A relative offset from a program address, used for jump instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstructionOffset(isize);

impl InstructionOffset {
    pub fn new(offset: isize) -> Self {
        Self(offset)
    }

    pub fn as_isize(&self) -> isize {
        self.0
    }
}

impl From<isize> for InstructionOffset {
    fn from(offset: isize) -> Self {
        Self(offset)
    }
}

impl Display for InstructionOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = match self.0.cmp(&0) {
            std::cmp::Ordering::Less => "-",
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => "+",
        };

        write!(f, "{}{:04}", sign, self.0.abs())
    }
}

#[cfg(test)]
mod tests {
    use super::InstructionOffset;

    #[test]
    fn should_display_positive_offset() {
        let offset = InstructionOffset::new(42);

        assert_eq!(format!("{}", offset), "+0042");
    }

    #[test]
    fn should_display_negative_offset() {
        let offset = InstructionOffset::new(-42);

        assert_eq!(format!("{}", offset), "-0042");
    }

    #[test]
    fn should_display_zero_offset() {
        let offset = InstructionOffset::new(0);

        assert_eq!(format!("{}", offset), "+0000");
    }
}
