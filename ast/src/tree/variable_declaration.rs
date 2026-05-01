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
