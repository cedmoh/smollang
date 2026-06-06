use crate::Expression;

/// A break expression, which is used to exit a loop.
///
/// # Examples
///
/// ```smollang
/// break
/// ```
///
/// ```smollang
/// break 5
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Break {
    pub expression: Option<Box<Expression>>,
}

impl Break {
    pub fn new(expression: Option<Expression>) -> Self {
        Self {
            expression: expression.map(Box::new),
        }
    }
}
