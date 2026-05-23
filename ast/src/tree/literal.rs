use crate::{Expression, Expressions};

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
    Array(ArrayLiteral),
    Object(ObjectLiteral),
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

impl From<StringLiteral> for Literal {
    fn from(string_literal: StringLiteral) -> Self {
        Literal::String(string_literal)
    }
}

impl StringLiteral {
    pub fn new(value: String) -> Self {
        Self { value }
    }
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

impl From<IntegerLiteral> for Literal {
    fn from(integer_literal: IntegerLiteral) -> Self {
        Literal::Integer(integer_literal)
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

impl From<DecimalLiteral> for Literal {
    fn from(decimal_literal: DecimalLiteral) -> Self {
        Literal::Decimal(decimal_literal)
    }
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

impl From<HexadecimalLiteral> for Literal {
    fn from(hexadecimal_literal: HexadecimalLiteral) -> Self {
        Literal::Hexadecimal(hexadecimal_literal)
    }
}

impl HexadecimalLiteral {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
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

impl From<BinaryLiteral> for Literal {
    fn from(binary_literal: BinaryLiteral) -> Self {
        Literal::Binary(binary_literal)
    }
}

impl BinaryLiteral {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
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

impl From<OctalLiteral> for Literal {
    fn from(octal_literal: OctalLiteral) -> Self {
        Literal::Octal(octal_literal)
    }
}

impl OctalLiteral {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

/// An array literal, which represents an array value.
///
/// # Examples
///     
/// ```smollang
/// [1, 2, 3]
/// ```
///
/// ```smollang
/// []  
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteral {
    pub elements: Expressions,
}

impl ArrayLiteral {
    pub fn new(elements: Expressions) -> Self {
        Self { elements }
    }

    pub fn builder() -> ArrayLiteralBuilder {
        ArrayLiteralBuilder::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteralBuilder {
    elements: Vec<Expression>,
}

impl ArrayLiteralBuilder {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: Expression) -> &mut Self {
        self.elements.push(element);
        self
    }

    pub fn with_element(mut self, element: Expression) -> Self {
        self.add_element(element);
        self
    }

    pub fn build(self) -> ArrayLiteral {
        ArrayLiteral::new(Expressions::new(self.elements))
    }
}

/// An object literal, which represents an object value.
///     
/// # Examples
///
/// ```smollang
/// {
///   name 'Alice',
///   age 30,
///   isStudent false,
///   salary getSalary(),
///   study || print 'Studying...',
/// }
/// ```
///
/// ```smollang
/// {}
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ObjectLiteral {
    pub properties: ObjectProperties,
}

impl ObjectLiteral {
    pub fn new(properties: ObjectProperties) -> Self {
        Self { properties }
    }

    pub fn builder() -> ObjectLiteralBuilder {
        ObjectLiteralBuilder::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLiteralBuilder {
    properties: Vec<ObjectProperty>,
}

impl ObjectLiteralBuilder {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }

    pub fn add_property(&mut self, property: ObjectProperty) -> &mut Self {
        self.properties.push(property);
        self
    }

    pub fn with_property(mut self, property: ObjectProperty) -> Self {
        self.add_property(property);
        self
    }

    pub fn build(self) -> ObjectLiteral {
        ObjectLiteral::new(ObjectProperties::new(self.properties))
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ObjectProperties {
    pub properties: Vec<ObjectProperty>,
}

impl ObjectProperties {
    pub fn new(properties: Vec<ObjectProperty>) -> Self {
        Self { properties }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectProperty {
    Shorthand(String),
    KeyValue(String, Expression),
}
