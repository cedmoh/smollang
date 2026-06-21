use crate::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub body: Box<Expression>,
}

impl Loop {
    pub fn new(body: impl Into<Expression>) -> Self {
        Self {
            body: Box::new(body.into()),
        }
    }
}
