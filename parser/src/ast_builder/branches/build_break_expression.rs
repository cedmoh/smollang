use crate::{
    ast_builder::{BuildAstExpressionError, build_ast_expression},
    rule_parser::Rule,
};
use ast::Break;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed break expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// broken_expression > expression > ...
/// ```
pub fn build_break_expression(
    pair: Pair<Rule>,
) -> Result<Break, BuildBreakExpressionError> {
    use BuildBreakExpressionError::*;

    let rule = pair.as_rule();

    if rule != Rule::broken_expression {
        return Err(RuleIsNotABreak(rule));
    }

    let mut inner_rules = pair.into_inner();

    let next_inner_rule = inner_rules.next();

    let Some(next_inner_rule) = next_inner_rule else {
        return Ok(Break::synthetic(None));
    };

    match build_ast_expression(next_inner_rule) {
        Ok(expr) => Ok(Break::synthetic(Some(expr))),
        Err(error) => Err(BuildExpressionError(error)),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildBreakExpressionError {
    /// The first rule is not a break expression.
    #[error("Expected a break expression, but found rule: {0:?}")]
    RuleIsNotABreak(Rule),

    /// An error occurred while building the break expression payload.
    #[error("An error occurred while building the break expression: {0}")]
    BuildExpressionError(BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::Identifier;

    #[test]
    fn should_build_break_expression_without_payload() {
        // Arrange
        let input = "br";

        let break_rule = parse_string_to_rule(input, Rule::broken_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a break expression.");

        // Act
        let break_expression = build_break_expression(break_rule);

        // Assert
        assert_eq!(break_expression, Ok(Break::synthetic(None)));
    }

    #[test]
    fn should_build_break_expression_with_payload() {
        // Arrange
        let input = "br value";

        let break_rule = parse_string_to_rule(input, Rule::broken_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a break expression.");

        // Act
        let break_expression = build_break_expression(break_rule);

        // Assert
        assert_eq!(
            break_expression,
            Ok(Break::synthetic(Some(
                Identifier::synthetic("value".to_string()).into()
            )))
        );
    }

    #[test]
    fn should_return_error_when_rule_is_not_break_expression() {
        // Arrange
        let input = "value";

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier.");

        // Act
        let break_expression = build_break_expression(identifier_rule);

        // Assert
        assert_eq!(
            break_expression,
            Err(BuildBreakExpressionError::RuleIsNotABreak(Rule::identifier,))
        );
    }
}
