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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block {
    /// The expressions in the block.
    pub body: Expressions,
}

impl Block {
    /// Creates a new empty block. To create a block with expressions, use the `builder` method, or the `BlockBuilder` directly.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a builder for creating a block with expressions.
    pub fn builder() -> BlockBuilder {
        BlockBuilder::new()
    }
}

/// A builder for creating a block expression.
pub struct BlockBuilder {
    /// The expressions in the block.
    expressions: Vec<Expression>,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new() -> Self {
        Self {
            expressions: Vec::new(),
        }
    }

    /// Adds an expression to the block.
    pub fn add_expression(&mut self, expression: Expression) -> &mut Self {
        self.expressions.push(expression);
        self
    }

    /// Builds the block expression.
    pub fn build(self) -> Block {
        Block {
            body: Expressions::new(self.expressions.clone()),
        }
    }
}
