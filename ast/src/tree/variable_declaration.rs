use super::*;

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
}

impl VariableDeclaration {
    /// Creates a new [`VariableDeclarationBuilder`] with the given variable
    /// name.
    ///
    /// # Examples
    ///
    /// ```
    /// use ast::{Identifier, VariableDeclaration};
    ///
    /// let declaration = VariableDeclaration::builder(Identifier::new("x".to_string()))
    ///     .with_mutability(true)
    ///     .build();
    ///
    /// assert_eq!(declaration.name, Identifier::new("x".to_string()));
    /// assert!(declaration.is_mutable);
    /// assert!(declaration.initial_value.is_none());
    /// ```
    pub fn builder(name: Identifier) -> VariableDeclarationBuilder {
        VariableDeclarationBuilder::new(name)
    }
}

pub struct VariableDeclarationBuilder {
    name: Identifier,
    is_mutable: bool,
    initial_value: Option<Box<Expression>>,
}

impl VariableDeclarationBuilder {
    pub fn new(name: Identifier) -> Self {
        Self {
            name,
            is_mutable: false,
            initial_value: None,
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

    pub fn with_initial_value(mut self, initial_value: Expression) -> Self {
        self.initial_value = Some(Box::new(initial_value));
        self
    }

    pub fn build(self) -> VariableDeclaration {
        VariableDeclaration {
            name: self.name,
            is_mutable: self.is_mutable,
            initial_value: self.initial_value,
        }
    }
}
