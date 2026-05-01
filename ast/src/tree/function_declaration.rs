use super::{Expression, Identifier};

/// Represents a function declaration.
///
/// # Examples
///     
/// ```
/// add |x,y| x + y
/// ```
///
/// ```
/// hello |name| (
///     print('Hello, ' + name)
/// )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Option<Identifier>,
    pub params: FunctionParameters,
    pub body: Option<FunctionBody>,
}

/// Represents a function parameter.
/// TODO: Add support for default parameter values.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub name: Identifier,
}

/// Represents the parameters of a function declaration.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FunctionParameters {
    pub items: Vec<FunctionParameter>,
}

/// Represents the body of a function declaration.
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
