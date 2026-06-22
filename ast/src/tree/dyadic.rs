use crate::{Expression, Span};

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
    /// The operator of the dyadic expression.
    pub operator: DyadicOperator,

    /// The left-hand side of the dyadic expression.
    pub left: Box<Expression>,

    /// The right-hand side of the dyadic expression.
    pub right: Box<Expression>,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl Dyadic {
    pub fn new(
        operator: DyadicOperator,
        left: Box<Expression>,
        right: Box<Expression>,
        span: Span,
    ) -> Self {
        Self {
            operator,
            left,
            right,
            span,
        }
    }

    /// Creates a synthetic dyadic expression with a dummy span.
    pub fn synthetic(
        operator: DyadicOperator,
        left: impl Into<Expression>,
        right: impl Into<Expression>,
    ) -> Self {
        Self::new(
            operator,
            Box::new(left.into()),
            Box::new(right.into()),
            Span::DUMMY,
        )
    }
}
