use crate::{Expression, Expressions, Span};

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
    Template(TemplateLiteral),
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
    /// The value of the boolean literal.
    pub value: bool,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl BooleanLiteral {
    pub fn new(value: bool, span: Span) -> Self {
        Self { value, span }
    }

    /// Creates a synthetic boolean literal with a dummy span.
    pub fn synthetic(value: bool) -> Self {
        Self::new(value, Span::DUMMY)
    }
}

/// A string literal, which represents a string value.
///     
/// # Examples
///
/// ```smollang
/// 'Hello, world!'
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    /// The value of the string literal.
    pub value: String,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl StringLiteral {
    pub fn new(value: String, span: Span) -> Self {
        Self { value, span }
    }

    /// Creates a synthetic string literal with a dummy span.
    pub fn synthetic(value: String) -> Self {
        Self::new(value, Span::DUMMY)
    }
}

/// A template literal, which represents a string value that can contain
/// embedded expressions.
///
/// # Examples
///     
/// ```smollang
/// $'Hello, {name}!'
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateLiteral {
    /// The value of the template literal, which is the raw string with
    /// embedded
    pub value: String,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl TemplateLiteral {
    pub fn new(value: String, span: Span) -> Self {
        Self { value, span }
    }

    /// Creates a synthetic template literal with a dummy span.
    pub fn synthetic(value: String) -> Self {
        Self::new(value, Span::DUMMY)
    }
}

impl From<TemplateLiteral> for Literal {
    fn from(template_literal: TemplateLiteral) -> Self {
        Literal::Template(template_literal)
    }
}

impl From<StringLiteral> for Literal {
    fn from(string_literal: StringLiteral) -> Self {
        Literal::String(string_literal)
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
    /// The value of the integer literal.
    pub value: i32,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl IntegerLiteral {
    pub fn new(value: i32, span: Span) -> Self {
        Self { value, span }
    }

    /// Creates a synthetic integer literal with a dummy span.
    pub fn synthetic(value: i32) -> Self {
        Self::new(value, Span::DUMMY)
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
    /// The value of the decimal literal.
    pub value: f64,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl From<DecimalLiteral> for Literal {
    fn from(decimal_literal: DecimalLiteral) -> Self {
        Literal::Decimal(decimal_literal)
    }
}

impl DecimalLiteral {
    pub fn new(value: f64, span: Span) -> Self {
        Self { value, span }
    }

    /// Creates a new decimal literal with the given value and span.
    pub fn synthetic(value: f64) -> Self {
        Self::new(value, Span::DUMMY)
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
    /// The value of the hexadecimal literal.
    pub value: i32,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl From<HexadecimalLiteral> for Literal {
    fn from(hexadecimal_literal: HexadecimalLiteral) -> Self {
        Literal::Hexadecimal(hexadecimal_literal)
    }
}

impl HexadecimalLiteral {
    pub fn new(value: i32, span: Span) -> Self {
        Self { value, span }
    }

    /// Creates a synthetic hexadecimal literal with a dummy span.
    pub fn synthetic(value: i32) -> Self {
        Self::new(value, Span::DUMMY)
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
    /// The value of the binary literal.
    pub value: i32,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl From<BinaryLiteral> for Literal {
    fn from(binary_literal: BinaryLiteral) -> Self {
        Literal::Binary(binary_literal)
    }
}

impl BinaryLiteral {
    pub fn new(value: i32, span: Span) -> Self {
        Self { value, span }
    }

    /// Creates a synthetic binary literal with a dummy span.
    pub fn synthetic(value: i32) -> Self {
        Self::new(value, Span::DUMMY)
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
    /// The value of the octal literal.
    pub value: i32,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl From<OctalLiteral> for Literal {
    fn from(octal_literal: OctalLiteral) -> Self {
        Literal::Octal(octal_literal)
    }
}

impl OctalLiteral {
    pub fn new(value: i32, span: Span) -> Self {
        Self { value, span }
    }

    /// Creates a new octal literal with the given value and span.
    pub fn synthetic(value: i32) -> Self {
        Self::new(value, Span::DUMMY)
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
    /// The elements of the array literal.
    pub elements: Expressions,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl ArrayLiteral {
    pub fn new(elements: Expressions, span: Span) -> Self {
        Self { elements, span }
    }

    pub fn synthetic(elements: Expressions) -> Self {
        Self::new(elements, Span::DUMMY)
    }

    pub fn builder() -> ArrayLiteralBuilder {
        ArrayLiteralBuilder::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteralBuilder {
    elements: Vec<Expression>,
    span: Option<Span>,
}

impl ArrayLiteralBuilder {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            span: None,
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

    pub fn span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn build(self) -> ArrayLiteral {
        ArrayLiteral::new(
            Expressions::new(self.elements),
            self.span.unwrap_or(Span::DUMMY),
        )
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
    /// The properties of the object literal.
    pub properties: ObjectProperties,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl ObjectLiteral {
    pub fn new(properties: ObjectProperties, span: Span) -> Self {
        Self { properties, span }
    }

    /// Creates a synthetic object literal with the given properties and a dummy
    /// span.
    pub fn synthetic(properties: ObjectProperties) -> Self {
        Self::new(properties, Span::DUMMY)
    }

    pub fn builder() -> ObjectLiteralBuilder {
        ObjectLiteralBuilder::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLiteralBuilder {
    properties: Vec<ObjectProperty>,
    span: Option<Span>,
}

impl ObjectLiteralBuilder {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            span: None,
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

    pub fn span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn build(self) -> ObjectLiteral {
        ObjectLiteral::new(
            ObjectProperties::synthetic(self.properties),
            self.span.unwrap_or(Span::DUMMY),
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ObjectProperties {
    /// The properties of the object literal.
    pub properties: Vec<ObjectProperty>,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl ObjectProperties {
    pub fn new(properties: Vec<ObjectProperty>, span: Span) -> Self {
        Self { properties, span }
    }

    pub fn synthetic(properties: Vec<ObjectProperty>) -> Self {
        Self::new(properties, Span::DUMMY)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectProperty {
    Shorthand(String),
    KeyValue(String, Expression),
}
