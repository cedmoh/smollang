use super::*;

/// A dyadic operator, which is an operator that takes two operands and performs a specific operation on them.
/// The following operators are supported:
/// - Arithmetic operators: `+`, `-`, `*`, `/`, `%`, `^`
/// - Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `
/// - Logical operators: `and`, `or`
/// - Range operators: `..`, `...`
#[derive(Debug, Clone, PartialEq)]
pub enum DyadicOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    RangeInclusive,
    Range,
}

/// A dyadic expression, which is an expression that consists of two sub-expressions and an operator.
///
/// # Examples
///
/// ```
/// true and false
/// ```
///
/// ```
/// x * y + z
/// ```
///
/// ```
/// 1..10
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Dyadic {
    pub operator: DyadicOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}
