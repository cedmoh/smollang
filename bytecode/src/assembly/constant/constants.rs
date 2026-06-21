use std::fmt::Display;

use crate::{Constant, ConstantAddress};

#[derive(Debug, Clone)]
pub struct Constants(Vec<Constant>);

impl Constants {
    pub fn new(constants: Vec<Constant>) -> Self {
        Self(constants)
    }

    pub fn get(&self, address: ConstantAddress) -> Option<&Constant> {
        self.0.get(address.as_usize())
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

impl Display for Constants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for constant in self.0.iter() {
            writeln!(f, "{}", constant)?;
        }

        Ok(())
    }
}
