use super::*;

/// An expression block, which is a sequence of expressions that are executed in order
/// and the value of the block is the value of the last expression in the block.
///
/// # Example
///
/// ```
/// x val (
///    2 + 2
/// )
/// ```
/// In this example, the block contains a single expression `2 + 2`, and the value of the block is `4`.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The expressions in the block
    pub body: Expressions,
}
