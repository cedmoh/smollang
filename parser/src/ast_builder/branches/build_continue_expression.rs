use crate::rule_parser::Rule;
use ast::Continue;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed continue expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// - continue_expression
/// ```
pub fn build_continue_expression(
    pair: Pair<Rule>,
) -> Result<Continue, BuildContinueExpressionError> {
    use BuildContinueExpressionError::*;

    let rule = pair.as_rule();

    if rule != Rule::continue_expression {
        return Err(RuleIsNotAContinue(rule));
    }

    Ok(Continue::new())
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildContinueExpressionError {
    /// The first rule is not a continue expression.
    #[error("Expected a continue expression, but found rule: {0:?}")]
    RuleIsNotAContinue(Rule),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;

    #[test]
    fn should_build_continue_expression() {
        // Arrange
        let input = "cont";

        let continue_rule =
            parse_string_to_rule(input, Rule::continue_expression)
                .expect("Expected input to be parsed to rules successfully.")
                .next()
                .expect("Expected input to contain a continue expression.");

        // Act
        let continue_expression = build_continue_expression(continue_rule);

        // Assert
        assert_eq!(continue_expression, Ok(Continue::new()));
    }

    #[test]
    fn should_return_error_when_rule_is_not_continue_expression() {
        // Arrange
        let input = "value";

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier.");

        // Act
        let continue_expression = build_continue_expression(identifier_rule);

        // Assert
        assert_eq!(
            continue_expression,
            Err(BuildContinueExpressionError::RuleIsNotAContinue(
                Rule::identifier,
            ))
        );
    }
}
