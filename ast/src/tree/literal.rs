use crate::{PrettyPrint, write_node_label, write_scalar_field};
use std::fmt;

/// A literal value, which is a piece of code that represents a constant value.
/// Literals include numbers, strings, booleans, and nil.
///
/// # Examples
///
/// ```
/// nil
/// ```
/// ```
/// true
/// ```
/// ```
/// "Hello, world!"
/// ```
/// ```
/// 5
/// ```
/// ```
/// 3.14
/// ```
/// ```
/// 0xFF
/// ```
/// ```
/// 0b1010
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Integer(IntegerLiteral),
    Decimal(DecimalLiteral),
    Hexadecimal(HexadecimalLiteral),
    Binary(BinaryLiteral),
    Octal(OctalLiteral),
}

/// A boolean literal, which represents a boolean value (true or false).
///
/// # Example
///
/// ```
/// true
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
}

/// A string literal, which represents a string value.
///     
/// # Example
///
/// ```
/// "Hello, world!"
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

/// An integer literal, which represents an integer value.
///     
/// # Example
///     
/// ```
/// 5
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct IntegerLiteral {
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

/// A decimal literal, which represents a decimal value.
///     
/// # Example
///
/// ```
/// 3.14
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DecimalLiteral {
    pub value: f64,
}

impl DecimalLiteral {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

/// A hexadecimal literal, which represents a hexadecimal value.
///     
/// # Example
///     
/// ```
/// 0xFF
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HexadecimalLiteral {
    pub value: i64,
}

/// A binary literal, which represents a binary value.
///   
/// # Example
///  
/// ```
/// 0b1010
/// ```   
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryLiteral {
    pub value: i64,
}

/// An octal literal, which represents an octal value.
///     
/// # Example
///     
/// ```
/// 0o77
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OctalLiteral {
    pub value: i64,
}

impl PrettyPrint for Literal {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        match self {
            Literal::Nil => write_node_label(f, indent, "nil"),
            Literal::Boolean(boolean_literal) => {
                boolean_literal.fmt_with_indent(f, indent)
            }
            Literal::String(string_literal) => {
                string_literal.fmt_with_indent(f, indent)
            }
            Literal::Integer(integer_literal) => {
                integer_literal.fmt_with_indent(f, indent)
            }
            Literal::Decimal(decimal_literal) => {
                decimal_literal.fmt_with_indent(f, indent)
            }
            Literal::Hexadecimal(hexadecimal_literal) => {
                hexadecimal_literal.fmt_with_indent(f, indent)
            }
            Literal::Binary(binary_literal) => {
                binary_literal.fmt_with_indent(f, indent)
            }
            Literal::Octal(octal_literal) => {
                octal_literal.fmt_with_indent(f, indent)
            }
        }
    }
}

impl PrettyPrint for BooleanLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "BooleanLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for StringLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "StringLiteral")?;
        write_scalar_field(f, indent + 2, "value", format!("'{}'", self.value))
    }
}

impl PrettyPrint for IntegerLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "IntegerLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for DecimalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "DecimalLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for HexadecimalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "HexadecimalLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for BinaryLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "BinaryLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}

impl PrettyPrint for OctalLiteral {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "OctalLiteral")?;
        write_scalar_field(f, indent + 2, "value", self.value)
    }
}
