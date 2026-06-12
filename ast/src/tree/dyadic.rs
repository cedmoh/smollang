use super::*;

/// A dyadic operator, which is an operator that takes two operands and performs
/// a specific operation on them. The following operators are supported:
/// - Arithmetic operators: `+`, `-`, `*`, `/`, `%`, `^`
/// - Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
/// - Logical operators: `and`, `or`
/// - Range operators: `..`, `..=`
/// - Member access operator: `.`
#[derive(Debug, Clone, PartialEq)]
pub enum DyadicOperator {
    Add,
    AddAssign,
    Subtract,
    SubtractAssign,
    Multiply,
    MultiplyAssign,
    Divide,
    DivideAssign,
    Modulo,
    ModuloAssign,
    Power,
    PowerAssign,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    AndAssign,
    Or,
    OrAssign,
    RangeInclusive,
    Range,
}

/// A dyadic expression, which is an expression that consists of two
/// sub-expressions and an operator.
///
/// # Examples
///
/// ```smollang
/// true and false
/// ```
///
/// ```smollang
/// x * y + z
/// ```
///
/// ```smollang
/// 1..10
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Dyadic {
    pub operator: DyadicOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

impl Dyadic {
    pub fn new(
        operator: DyadicOperator,
        left: impl Into<Expression>,
        right: impl Into<Expression>,
    ) -> Self {
        Self {
            operator,
            left: Box::new(left.into()),
            right: Box::new(right.into()),
        }
    }
}
