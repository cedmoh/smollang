use super::Expressions;

/// A member expression, which represents accessing a member of an object or a module.
///     
/// # Example
///
/// ```
/// math.sin(3.14)
/// ```
/// In this example, `math.sin` is a member expression that accesses the `sin` member of the `math` module.
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    ///
    pub chain: Expressions,
}
