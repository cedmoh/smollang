use crate::Expression;

/// A pipe expression, which allows for chaining multiple expressions together.
///
/// # Examples
///
/// ```smollang
/// 1 |> add(it, 2) |> multiply(it, 3)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Pipe {
    pub arms: PipeArms,
}

impl Pipe {
    pub fn new(arms: PipeArms) -> Self {
        Self { arms }
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
    pub expression: Expression,
}

impl PipeArm {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PipeArms {
    pub arms: Vec<PipeArm>,
}

impl PipeArms {
    pub fn new(arms: Vec<PipeArm>) -> Self {
        Self { arms }
    }

    pub fn add_arm(&mut self, arm: PipeArm) -> &mut Self {
        self.arms.push(arm);
        self
    }
}

pub struct PipeBuilder {
    arms: PipeArms,
}

impl PipeBuilder {
    pub fn new(
        first_expression: Expression,
        second_expression: Expression,
    ) -> Self {
        Self {
            arms: PipeArms::new(vec![
                PipeArm::new(first_expression),
                PipeArm::new(second_expression),
            ]),
        }
    }

    pub fn with_arm(mut self, expression: Expression) -> Self {
        self.arms.add_arm(PipeArm::new(expression));
        self
    }

    pub fn add_arm(&mut self, expression: Expression) -> &mut Self {
        self.arms.add_arm(PipeArm::new(expression));
        self
    }

    pub fn build(self) -> Pipe {
        Pipe::new(self.arms)
    }
}
