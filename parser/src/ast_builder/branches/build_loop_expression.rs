use crate::{
    ast_builder::{BuildAstExpressionError, build_ast_expression},
    rule_parser::Rule,
};
use ast::Loop;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed loop expression into an AST
/// representation.
///     
/// # Examples
///
/// ```pest
/// loop_expression > loop_body > expression > ...
/// ```
pub fn build_loop_expression(
    pair: Pair<'_, Rule>,
) -> Result<Loop, BuildLoopExpressionError> {
    use BuildLoopExpressionError::*;
    use Rule::{loop_body, loop_expression};

    let rule = pair.as_rule();

    if rule != loop_expression {
        return Err(RuleIsNotALoopExpression(rule));
    }

    let mut inner = pair.into_inner();

    let body_pair = inner.next().ok_or(MissingLoopBody)?;

    if body_pair.as_rule() != loop_body {
        return Err(InvalidLoopBodyRule(body_pair.as_rule()));
    }

    let body_expression_pair =
        body_pair.into_inner().next().ok_or(EmptyLoopBody)?;
    let body = build_ast_expression(body_expression_pair)?;

    Ok(Loop::new(body))
}
#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildLoopExpressionError {
    /// The first rule is not a loop expression.
    #[error("Expected a loop expression, but found rule: {0:?}")]
    RuleIsNotALoopExpression(Rule),

    /// The loop expression is missing its body.
    #[error("Missing loop body in loop expression")]
    MissingLoopBody,

    /// The loop body rule is not a `loop_body`.
    #[error("Expected a loop_body rule, but found rule: {0:?}")]
    InvalidLoopBodyRule(Rule),

    /// The loop body exists but does not contain an expression.
    #[error("The loop body expression is empty")]
    EmptyLoopBody,

    /// An error occurred while building the loop body expression.
    #[error("An error occurred while building the loop body expression: {0}")]
    BuildAstExpressionError(#[from] BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::Identifier;

    #[test]
    fn should_build_loop_expression_with_identifier_body() {
        // Arrange
        let input = "loop body";

        let loop_rule = parse_string_to_rule(input, Rule::loop_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a loop expression.");

        // Act
        let loop_expression = build_loop_expression(loop_rule);

        // Assert
        assert_eq!(
            loop_expression,
            Ok(Loop::new(Identifier::new("body".to_string()).into()))
        );
    }

    #[test]
    fn should_return_error_when_rule_is_not_loop_expression() {
        // Arrange
        let input = "value";

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier.");

        // Act
        let loop_expression = build_loop_expression(identifier_rule);

        // Assert
        assert_eq!(
            loop_expression,
            Err(BuildLoopExpressionError::RuleIsNotALoopExpression(
                Rule::identifier
            ))
        );
    }
}
