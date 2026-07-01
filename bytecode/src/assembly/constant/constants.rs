use std::{fmt::Display, ops::Index};

use crate::{Constant, ConstantAddress};

#[derive(Debug, Clone)]
pub struct Constants(Vec<Constant>);

impl Constants {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get(&self, address: ConstantAddress) -> Option<&Constant> {
        self.0.get(address.as_usize())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Constant> {
        self.0.iter()
    }

    pub fn push(&mut self, constant: Constant) {
        self.0.push(constant);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<Constant>> for Constants {
    fn from(constants: Vec<Constant>) -> Self {
        Self(constants)
    }
}

impl Into<Vec<Constant>> for Constants {
    fn into(self) -> Vec<Constant> {
        self.0
    }
}

impl Index<ConstantAddress> for Constants {
    type Output = Constant;

    fn index(&self, index: ConstantAddress) -> &Self::Output {
        &self.0[index.as_usize()]
    }
}

impl Display for Constants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for constant in self.0.iter() {
            writeln!(f, "{}", constant)?;
        }

        Ok(())
    }
}
