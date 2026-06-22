use crate::{Expression, Expressions, Span};

/// An expression block, which is a sequence of expressions that are executed in
/// order and the value of the block is the value of the last expression in the
/// block.
///
/// # Examples
///
/// ```smollang
/// x val (
///    2 + 2
/// )
/// ```
/// In this example, the block contains a single expression `2 + 2`, and the
/// value of the block is `4`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block {
    /// The expressions in the block.
    pub body: Expressions,

    /// The location of the AST node in the source code
    pub span: Span,
}

impl Block {
    /// Creates a new empty block. To create a block with expressions, use the
    /// `builder` method, or the `BlockBuilder` directly.
    pub fn synthetic() -> Self {
        Self::default()
    }

    pub fn new(body: Expressions, span: Span) -> Self {
        Self { body, span }
    }

    /// Returns a builder for creating a block with expressions.
    pub fn builder() -> BlockBuilder {
        BlockBuilder::new()
    }
}

/// A builder for creating a block expression.
pub struct BlockBuilder {
    /// The expressions in the block.
    expressions: Expressions,

    /// The location of the AST node in the source code
    span: Option<Span>,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new() -> Self {
        Self {
            expressions: Expressions::default(),
            span: None,
        }
    }

    /// Adds an expression to the block.
    pub fn add_expression(&mut self, expression: Expression) -> &mut Self {
        self.expressions.add_expression(expression);
        self
    }

    /// Sets the span for the block.
    pub fn span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Builds the block expression.
    pub fn build(self) -> Block {
        Block::new(self.expressions, self.span.unwrap_or(Span::DUMMY))
    }
}
