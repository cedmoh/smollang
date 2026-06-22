use crate::{into_ast_span::IntoAstSpan, rule_parser::Rule};
use ast::BooleanLiteral;
use pest::iterators::Pair;
use thiserror::Error;

/// Builds a boolean literal from a pest rule.
///
/// ```pest
/// boolean_literal > true: "true"
/// ```
/// ```pest
/// boolean_literal > false: "false"
/// ```
pub fn build_boolean_literal(
    pair: Pair<Rule>,
) -> Result<BooleanLiteral, BuildBooleanLiteralError> {
    use BuildBooleanLiteralError::*;
    use Rule::boolean_literal;

    let rule = pair.as_rule();

    if rule != boolean_literal {
        return Err(UnexpectedRule(rule));
    }

    let span = pair.as_span().into_ast_span();

    let value = match pair.as_str() {
        "true" => true,
        "false" => false,
        other => return Err(UnexpectedBooleanText(other.to_string())),
    };

    Ok(BooleanLiteral::new(value, span))
}

#[derive(Debug, Error)]
pub enum BuildBooleanLiteralError {
    #[error("Expected a boolean_literal, but found {0:?}")]
    UnexpectedRule(Rule),

    #[error("Expected 'true' or 'false', but found '{0}'")]
    UnexpectedBooleanText(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};
    use ast::Span;

    #[test]
    fn should_build_true_when_input_is_true() {
        // Arrange
        let input = "true";

        let pair = parse_string_to_rule(input, Rule::boolean_literal)
            .expect("Failed to parse boolean_literal")
            .next()
            .expect("Expected at least one pair for boolean_literal");

        // Act
        let result = build_boolean_literal(pair);

        // Assert
        let result = result.expect("Asserted successful build of boolean_literal");
        assert!(result.value);
        assert_ne!(result.span, Span::DUMMY);
    }

    #[test]
    fn should_build_false_when_input_is_false() {
        // Arrange
        let input = "false";

        let pair = parse_string_to_rule(input, Rule::boolean_literal)
            .expect("Failed to parse boolean_literal")
            .next()
            .expect("Expected at least one pair for boolean_literal");

        // Act
        let result = build_boolean_literal(pair);

        // Assert
        let result = result.expect("Asserted successful build of boolean_literal");
        assert!(!result.value);
        assert_ne!(result.span, Span::DUMMY);
    }
}
