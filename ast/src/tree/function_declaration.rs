use super::{Expression, Identifier};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Option<Identifier>,
    pub params: FunctionParameters,
    pub body: Option<FunctionBody>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub name: Identifier,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionParameters {
    pub items: Vec<FunctionParameter>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub body: Box<Expression>,
}

/// A return expression, which is used to return a value from a function.
///
/// # Example
///
/// ```
/// ret 5
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub xp: Option<Box<Expression>>,
}
