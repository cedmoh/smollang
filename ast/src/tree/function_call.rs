use super::{Expression, Expressions};

/// Represents a function call.
///
/// # Examples
///
/// ```
/// print 'Hello, World!'
/// ```
///
/// ```
/// add(1, 2)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    /// The expression representing the function being called.
    pub callee: Box<Expression>,

    /// The arguments passed to the function call.
    pub arguments: CallArguments,
}

/// Represents the arguments passed to a function call.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CallArguments {
    pub expressions: Expressions,
}
