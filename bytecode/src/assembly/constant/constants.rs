use std::{fmt::Display, ops::Index};

use crate::{Constant, ConstantAddress};

#[derive(Debug, Clone)]
pub struct Constants(Vec<Constant>);

impl Constants {
    pub fn new(constants: Vec<Constant>) -> Self {
        Self(constants)
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
