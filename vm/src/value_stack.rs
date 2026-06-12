use bytecode::Value;

use std::{
    fmt::Display,
    ops::{Index, Range, RangeFull},
};
use thiserror::Error;

#[derive(Debug)]
pub struct ValueStack {
    values: Vec<Value>,
}

impl ValueStack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn duplicate(&mut self) -> Result<(), ValueStackError> {
        let value =
            self.values.last().ok_or(ValueStackError::StackUnderflow)?;

        self.values.push(value.clone());

        Ok(())
    }

    pub fn duplicate_two(&mut self) -> Result<(), ValueStackError> {
        let len = self.values.len();
        let top = self
            .values
            .get(len - 1)
            .ok_or(ValueStackError::StackUnderflow)?;
        let bottom = self
            .values
            .get(len - 2)
            .ok_or(ValueStackError::StackUnderflow)?;

        let top_clone = top.clone();
        let bottom_clone = bottom.clone();

        // Push the bottom value first, then the top value,
        // to maintain the correct order on the stack.
        self.values.push(bottom_clone);
        self.values.push(top_clone);

        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value, ValueStackError> {
        self.values.pop().ok_or(ValueStackError::StackUnderflow)
    }

    pub fn last(&self) -> Option<&Value> {
        self.values.last()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for ValueStack {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl Index<Range<usize>> for ValueStack {
    type Output = [Value];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.values[index]
    }
}

impl Index<RangeFull> for ValueStack {
    type Output = [Value];

    fn index(&self, _: std::ops::RangeFull) -> &Self::Output {
        &self.values[..]
    }
}

impl Display for ValueStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- Beginning of Stack ---")?;

        for (i, value) in self.values.iter().enumerate() {
            writeln!(f, "{:0>4}: {}", i, value)?;
        }

        writeln!(f, "--- End of Stack ---")
    }
}

#[derive(Debug, Error)]
pub enum ValueStackError {
    /// Returned when an instruction that requires popping a value from the stack is executed,
    /// but the stack does not contain enough values to pop.
    #[error("Attempted to pop value from empty stack")]
    StackUnderflow,
}
