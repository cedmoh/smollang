use crate::{PrettyPrint, write_field_label, write_node_label};

use super::*;

/// A dyadic operator, which is an operator that takes two operands and performs
/// a specific operation on them. The following operators are supported:
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
        left: Expression,
        right: Expression,
    ) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl PrettyPrint for DyadicOperator {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        let label = match self {
            DyadicOperator::Add => "Add",
            DyadicOperator::Subtract => "Subtract",
            DyadicOperator::Multiply => "Multiply",
            DyadicOperator::Divide => "Divide",
            DyadicOperator::Modulo => "Modulo",
            DyadicOperator::Power => "Power",
            DyadicOperator::Equal => "Equal",
            DyadicOperator::NotEqual => "NotEqual",
            DyadicOperator::LessThan => "LessThan",
            DyadicOperator::GreaterThan => "GreaterThan",
            DyadicOperator::LessThanOrEqual => "LessThanOrEqual",
            DyadicOperator::GreaterThanOrEqual => "GreaterThanOrEqual",
            DyadicOperator::And => "And",
            DyadicOperator::Or => "Or",
            DyadicOperator::RangeInclusive => "RangeInclusive",
            DyadicOperator::Range => "Range",
        };

        write_node_label(f, indent, label)
    }
}

impl PrettyPrint for Dyadic {
    fn fmt_with_indent(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent: usize,
    ) -> std::fmt::Result {
        write_node_label(f, indent, "Dyadic")?;
        write_field_label(f, indent, "operator")?;
        self.operator.fmt_with_indent(f, indent + 2)?;
        write_field_label(f, indent, "left")?;
        self.left.fmt_with_indent(f, indent + 2)?;
        write_field_label(f, indent, "right")?;
        self.right.fmt_with_indent(f, indent + 2)
    }
}
