use crate::{
    BinaryLiteral, BooleanLiteral, DecimalLiteral, HexadecimalLiteral,
    Identifier, IntegerLiteral, OctalLiteral, StringLiteral,
};

/// A pattern, which is used in match expressions to specify the structure of
/// the value being matched.
///
/// # Examples
///
/// ```smollang
/// [x, y]
/// ```
/// In this example, `[x, y]` is a pattern that matches a tuple with two
/// elements, where the first element is bound to `x` and the second element is
/// bound to `y`.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// A literal pattern, which matches a specific literal value.
    ///
    /// # Examples
    ///
    /// ```smollang
    /// 42
    /// ```
    ///
    /// ```smollang
    /// 'hello'
    /// ```
    ///
    /// ```smollang
    /// true
    /// ```
    Literal(LiteralPattern),
    /// A variable pattern, which matches any value and binds it to a variable.
    ///
    /// # Examples
    ///
    /// ```smollang
    /// x
    /// ```
    Identifier(IdentifierPattern),
    /// An array pattern, which matches an array with a specific structure.
    ///
    /// # Examples
    ///
    /// ```smollang
    /// [x, y]
    /// ```
    Array(Vec<ArrayPatternElement>),
    /// A destructuring pattern, which matches a struct with a specific
    /// structure.
    ///
    /// # Examples
    ///
    /// ```smollang
    /// { x, y }
    /// ```
    Destructuring(Vec<DestructuringPatternElement>),
    /// A wildcard pattern, which matches any value but does not bind it to a
    /// variable.
    ///
    /// # Examples
    ///
    /// ```smollang
    /// _
    /// ```
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralPattern {
    Nil,
    Boolean(BooleanLiteral),
    String(StringLiteral),
    Integer(IntegerLiteral),
    Decimal(DecimalLiteral),
    Hexadecimal(HexadecimalLiteral),
    Binary(BinaryLiteral),
    Octal(OctalLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierPattern(pub Identifier);

impl IdentifierPattern {
    pub fn new(name: Identifier) -> Self {
        Self(name)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArrayPattern {
    pub items: Vec<ArrayPatternElement>,
}

impl ArrayPattern {
    pub fn new(items: Vec<ArrayPatternElement>) -> Self {
        Self { items }
    }

    pub fn builder() -> ArrayPatternBuilder {
        ArrayPatternBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct ArrayPatternBuilder {
    items: Vec<ArrayPatternElement>,
}

impl ArrayPatternBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pattern(&mut self, pattern: Pattern) -> &mut Self {
        self.items.push(ArrayPatternElement::Pattern(pattern));
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.add_pattern(pattern);
        self
    }

    pub fn add_rest(&mut self) -> &mut Self {
        self.items.push(ArrayPatternElement::Rest);
        self
    }

    pub fn with_rest(mut self) -> Self {
        self.add_rest();
        self
    }

    pub fn build(self) -> ArrayPattern {
        ArrayPattern::new(self.items)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayPatternElement {
    Pattern(Pattern),
    Rest,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DestructuringPattern {
    pub fields: Vec<DestructuringPatternElement>,
}

impl DestructuringPattern {
    pub fn new(fields: Vec<DestructuringPatternElement>) -> Self {
        Self { fields }
    }

    pub fn builder() -> DestructuringPatternBuilder {
        DestructuringPatternBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct DestructuringPatternBuilder {
    fields: Vec<DestructuringPatternElement>,
}

impl DestructuringPatternBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_field(
        &mut self,
        name: Identifier,
        pattern: Option<Pattern>,
    ) -> &mut Self {
        self.fields
            .push(DestructuringPatternElement::new(name, pattern));
        self
    }

    pub fn with_field(
        mut self,
        name: Identifier,
        pattern: Option<Pattern>,
    ) -> Self {
        self.add_field(name, pattern);
        self
    }

    pub fn build(self) -> DestructuringPattern {
        DestructuringPattern::new(self.fields)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DestructuringPatternElement {
    pub name: Identifier,
    pub pattern: Option<Pattern>,
}

impl DestructuringPatternElement {
    pub fn new(name: Identifier, pattern: Option<Pattern>) -> Self {
        Self { name, pattern }
    }
}
