use crate::Expression;

/// A return expression, which is used to return a value from a function.
///
/// # Examples
///
/// ```smollang
/// ret 5
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub expression: Option<Box<Expression>>,
}

impl Return {
    pub fn new(expression: Option<Expression>) -> Self {
        Self {
            expression: expression.map(Box::new),
        }
    }
}
