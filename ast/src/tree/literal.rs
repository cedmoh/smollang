/// A literal value, which is a piece of code that represents a constant value.
/// Literals include numbers, strings, booleans, and nil.
///
/// # Examples
///
/// ```smollang
/// nil
/// ```
/// ```smollang
/// true
/// ```
/// ```smollang
/// "Hello, world!"
/// ```
/// ```smollang
/// 5
/// ```
/// ```smollang
/// 3.14
/// ```
/// ```smollang
/// 0xFF
/// ```
/// ```smollang
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
/// # Examples
///
/// ```smollang
/// true
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
}

/// A string literal, which represents a string value.
///     
/// # Examples
///
/// ```smollang
/// "Hello, world!"
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

/// An integer literal, which represents an integer value.
///     
/// # Examples
///     
/// ```smollang
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
/// # Examples
///
/// ```smollang
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
/// # Examples
///     
/// ```smollang
/// 0xFF
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HexadecimalLiteral {
    pub value: i64,
}

/// A binary literal, which represents a binary value.
///   
/// # Examples
///  
/// ```smollang
/// 0b1010
/// ```   
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryLiteral {
    pub value: i64,
}

/// An octal literal, which represents an octal value.
///     
/// # Examples
///     
/// ```smollang
/// 0o77
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OctalLiteral {
    pub value: i64,
}
