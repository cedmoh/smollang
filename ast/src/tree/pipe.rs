use crate::{Expression, Span};

/// A pipe expression, which allows for chaining multiple expressions together.
///
/// # Examples
///
/// ```smollang
/// 1 |> add(it, 2) |> multiply(it, 3)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Pipe {
    /// The arms of the pipe expression, which represent the individual
    /// expressions being chained together.
    pub arms: PipeArms,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl Pipe {
    pub fn new(arms: PipeArms, span: Span) -> Self {
        Self { arms, span }
    }

    /// Creates a synthetic pipe expression with a dummy span.
    pub fn synthetic(arms: PipeArms) -> Self {
        Self::new(arms, Span::DUMMY)
    }

    pub fn builder(
        first_expression: Expression,
        second_expression: Expression,
    ) -> PipeBuilder {
        PipeBuilder::new(first_expression, second_expression)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PipeArm {
    /// The expression representing the individual arm of the pipe.
    pub expression: Expression,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl PipeArm {
    pub fn new(expression: Expression, span: Span) -> Self {
        Self { expression, span }
    }

    /// Creates a synthetic pipe arm with a dummy span.
    pub fn synthetic(expression: Expression) -> Self {
        Self::new(expression, Span::DUMMY)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PipeArms {
    /// The individual arms of the pipe expression.
    pub arms: Vec<PipeArm>,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl PipeArms {
    pub fn new(arms: Vec<PipeArm>, span: Span) -> Self {
        Self { arms, span }
    }

    pub fn synthetic(arms: Vec<PipeArm>) -> Self {
        Self::new(arms, Span::DUMMY)
    }

    pub fn add_arm(&mut self, arm: PipeArm) -> &mut Self {
        self.arms.push(arm);
        self
    }
}

pub struct PipeBuilder {
    arms: PipeArms,
    span: Option<Span>,
}

impl PipeBuilder {
    pub fn new(
        first_expression: Expression,
        second_expression: Expression,
    ) -> Self {
        Self {
            arms: PipeArms::synthetic(vec![
                PipeArm::synthetic(first_expression),
                PipeArm::synthetic(second_expression),
            ]),
            span: None,
        }
    }

    pub fn with_arm(mut self, expression: Expression) -> Self {
        self.arms.add_arm(PipeArm::synthetic(expression));
        self
    }

    pub fn add_arm(&mut self, expression: Expression) -> &mut Self {
        self.arms.add_arm(PipeArm::synthetic(expression));
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn span(&mut self, span: Span) -> &mut Self {
        self.span = Some(span);
        self
    }

    pub fn build(self) -> Pipe {
        Pipe::new(self.arms, self.span.unwrap_or(Span::DUMMY))
    }
}
