use crate::rule_parser::Rule;
use ast::Literal;
use pest::iterators::Pair;
use thiserror::Error;

/// Builds a nil literal from a pest rule.
///
/// ```pest
/// nil_literal > nil: "nil"
/// ```
pub fn build_nil_literal(
    pair: Pair<Rule>,
) -> Result<Literal, BuildNilLiteralError> {
    use Rule::nil_literal;

    let rule = pair.as_rule();

    if rule != nil_literal {
        return Err(BuildNilLiteralError::UnexpectedRule(rule));
    }

    Ok(Literal::Nil)
}

#[derive(Debug, Error)]
pub enum BuildNilLiteralError {
    #[error("Expected a nil_literal, but found {0:?}")]
    UnexpectedRule(Rule),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};

    #[test]
    fn should_build_nil_literal() {
        // Arrange
        let input = "nil";

        let pair = parse_string_to_rule(input, Rule::nil_literal)
            .expect("Failed to parse nil_literal")
            .next()
            .expect("Expected at least one pair for nil_literal");

        // Act
        let result = build_nil_literal(pair);

        // Assert
        assert_eq!(
            result.expect("Asserted successful build of nil_literal"),
            Literal::Nil
        );
    }

    #[test]
    fn should_return_error_when_rule_is_not_nil_literal() {
        // Arrange
        let input = r#"'hello'"#;

        let pair = parse_string_to_rule(input, Rule::string_literal)
            .expect("Failed to parse string_literal")
            .next()
            .expect("Expected at least one pair for string_literal");

        // Act
        let result = build_nil_literal(pair);

        // Assert
        assert!(result.is_err());
    }
}
