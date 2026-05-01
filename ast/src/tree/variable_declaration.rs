use super::{Expression, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    /// The name of the variable being declared.
    pub name: Identifier,

    /// Whether the variable is mutable (i.e., can be reassigned after its initial declaration).
    pub is_mutable: bool,

    /// An optional initial value for the variable. If provided, this expression will be evaluated and assigned to the variable upon declaration.
    pub initial_value: Option<Box<Expression>>,
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

    pub fn mutable(mut self) -> Self {
        self.is_mutable = true;
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
