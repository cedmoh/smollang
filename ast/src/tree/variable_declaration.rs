use crate::{Expression, Identifier, Span};

/// # Examples
///
/// Mutable variable declaration without initial value:
///
/// ```smollang
/// x var
/// ```
///
/// Mutable variable declaration with initial value:
///
/// ```smollang
/// x var 5
/// ```
///
/// Immutable variable declaration with initial value:
///
/// ```smollang
/// x val (5 + 3)
/// ```

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    /// The name of the variable being declared.
    pub name: Identifier,

    /// Whether the variable is mutable (i.e., can be reassigned after its
    /// initial declaration).
    pub is_mutable: bool,

    /// An optional initial value for the variable. If provided, this
    /// expression will be evaluated and assigned to the variable upon
    /// declaration.
    pub initial_value: Option<Box<Expression>>,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl VariableDeclaration {
    pub fn new(
        name: Identifier,
        is_mutable: bool,
        initial_value: Option<Box<Expression>>,
        span: Span,
    ) -> Self {
        Self {
            name,
            is_mutable,
            initial_value,
            span,
        }
    }

    /// Creates a synthetic variable declaration with a dummy span.
    pub fn synthetic(
        name: Identifier,
        is_mutable: bool,
        initial_value: Option<Box<Expression>>,
    ) -> Self {
        Self::new(name, is_mutable, initial_value, Span::DUMMY)
    }

    /// Creates a new [`VariableDeclarationBuilder`] with the given variable
    /// name.
    ///
    /// # Examples
    ///
    /// ```
    /// use ast::{Identifier, VariableDeclaration};
    ///
    /// let declaration = VariableDeclaration::builder("x")
    ///     .with_mutability(true)
    ///     .build();
    ///
    /// assert_eq!(declaration.name, Identifier::new("x".to_string()));
    /// assert!(declaration.is_mutable);
    /// assert!(declaration.initial_value.is_none());
    /// ```
    pub fn builder(name: impl Into<Identifier>) -> VariableDeclarationBuilder {
        VariableDeclarationBuilder::new(name.into())
    }
}

pub struct VariableDeclarationBuilder {
    name: Identifier,
    is_mutable: bool,
    initial_value: Option<Box<Expression>>,
    span: Option<Span>,
}

impl VariableDeclarationBuilder {
    pub fn new(name: Identifier) -> Self {
        Self {
            name,
            is_mutable: false,
            initial_value: None,
            span: None,
        }
    }

    pub fn mutable(&mut self) -> &mut Self {
        self.is_mutable = true;
        self
    }

    pub fn with_mutability(mut self, is_mutable: bool) -> Self {
        self.is_mutable = is_mutable;
        self
    }

    pub fn with_initial_value(
        mut self,
        initial_value: impl Into<Expression>,
    ) -> Self {
        self.initial_value = Some(Box::new(initial_value.into()));
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    pub fn build(self) -> VariableDeclaration {
        VariableDeclaration::synthetic(
            self.name,
            self.is_mutable,
            self.initial_value,
        )
    }
}
