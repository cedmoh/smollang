use crate::rule_parser::Rule;
use ast::OctalLiteral;
use pest::iterators::Pair;
use thiserror::Error;

/// Builds an octal literal from a pest rule.
///
/// The octal literal uses the prefix `o` followed by octal digits,
/// e.g. `o77` represents the decimal value `63`.
///
/// ```pest
/// octal_literal: "o77"
/// ```
pub fn build_octal_literal(
    pair: Pair<Rule>,
) -> Result<OctalLiteral, BuildOctalLiteralError> {
    use BuildOctalLiteralError::*;
    use Rule::octal_literal;

    let rule = pair.as_rule();

    if rule != octal_literal {
        return Err(UnexpectedRule(rule));
    }

    let text = pair.as_str();
    let digits = text
        .strip_prefix('o')
        .ok_or_else(|| ExpectedOctalPrefix(text.to_string()))?
        .trim()
        .replace('_', "");

    let value = i32::from_str_radix(&digits, 8)
        .map_err(|_| ParseFailed(text.to_string()))?;

    Ok(OctalLiteral::new(value))
}

#[derive(Debug, Error)]
pub enum BuildOctalLiteralError {
    #[error("Expected an octal_literal, but found {0:?}")]
    UnexpectedRule(Rule),

    #[error(
        "Expected octal_literal to start with 'o', but it was missing in '{0}'"
    )]
    ExpectedOctalPrefix(String),

    #[error("Failed to parse '{0}' as an octal number")]
    ParseFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};

    #[test]
    fn should_build_octal_literal() {
        // Arrange
        let input = "o77";

        let pair = parse_string_to_rule(input, Rule::octal_literal)
            .expect("Failed to parse octal_literal")
            .next()
            .expect("Expected at least one pair for octal_literal");

        // Act
        let result = build_octal_literal(pair);

        // Assert
        assert_eq!(
            result.expect("Asserted successful build of octal_literal"),
            OctalLiteral::new(0o77)
        );
    }

    #[test]
    fn should_build_octal_literal_ignoring_underscores() {
        // Arrange
        let input = "o7_7";

        let pair = parse_string_to_rule(input, Rule::octal_literal)
            .expect("Failed to parse octal_literal")
            .next()
            .expect("Expected at least one pair for octal_literal");

        // Act
        let result = build_octal_literal(pair);

        // Assert
        assert_eq!(
            result.expect("Asserted successful build of octal_literal"),
            OctalLiteral::new(0o77)
        );
    }
}
