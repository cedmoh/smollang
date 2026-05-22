use crate::Use;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Directives {
    pub items: Vec<Directive>,
}

impl Directives {
    pub fn new(items: Vec<Directive>) -> Self {
        Self { items }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Directive {
    Use(Use),
}
