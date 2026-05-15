use crate::{Expression, write_field_label};
use crate::{PrettyPrint, write_node_label, write_scalar_field};
use std::fmt;

/// A pipe expression, which allows for chaining multiple expressions together.
///
/// # Example
///
/// ```
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

impl PrettyPrint for Pipe {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        write_node_label(f, indent, "Pipe")?;
        write_field_label(f, indent + 2, "arms")?;
        self.arms.fmt_with_indent(f, indent + 4)
    }
}

impl PrettyPrint for PipeArm {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        self.expression.fmt_with_indent(f, indent)
    }
}

impl PrettyPrint for PipeArms {
    fn fmt_with_indent(
        &self,
        f: &mut fmt::Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        for arm in &self.arms {
            arm.fmt_with_indent(f, indent)?;
        }

        Ok(())
    }
}
