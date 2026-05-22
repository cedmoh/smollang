use super::*;

/// A member expression, which represents accessing a member of an object or a
/// module.     
///
/// # Examples
///
/// ```smollang
/// math.sin(3.14)
/// ```
/// In this example, `math.sin` is a member expression that accesses the `sin`
/// member of the `math` module.
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub object: Box<Expression>,
    pub property: Box<Expression>,
}

impl Member {
    pub fn new(object: Expression, property: Expression) -> Self {
        Self {
            object: Box::new(object),
            property: Box::new(property),
        }
    }
}
