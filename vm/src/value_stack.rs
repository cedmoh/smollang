use std::{
    fmt::Display,
    ops::{Index, Range, RangeFull},
};

use crate::value::Value;

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

    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
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
