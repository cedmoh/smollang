use super::Expressions;

/// A program represents a File. It consists of a sequence of expressions that will be executed in order.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// The body of the program, which consists of a sequence of expressions that will be executed in order.
    pub body: Expressions,
}
