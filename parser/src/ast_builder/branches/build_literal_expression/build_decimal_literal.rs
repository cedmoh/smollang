use crate::rule_parser::Rule;
use ast::{DecimalLiteral, Literal};
use pest::iterators::Pair;
use thiserror::Error;

/// Builds a [`DecimalLiteral`] from a pest rule.
///
/// ```pest
/// decimal_literal: "3.14"
/// ```
pub fn build_decimal_literal(
    pair: Pair<Rule>,
) -> Result<Literal, BuildDecimalLiteralError> {
    use BuildDecimalLiteralError::*;
    use Rule::decimal_literal;

    let rule = pair.as_rule();

    if rule != decimal_literal {
        return Err(UnexpectedRule(rule));
    }

    let text = pair.as_str().trim().replace('_', "");

    let value: f64 =
        text.parse().map_err(|_| ParseDecimalFailed(text.clone()))?;
    Ok(DecimalLiteral::new(value).into())
}

#[derive(Debug, Error)]
pub enum BuildDecimalLiteralError {
    #[error("Expected a decimal_literal, but found {0:?}")]
    UnexpectedRule(Rule),

    #[error("Failed to parse '{0}' as a decimal number")]
    ParseDecimalFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};

    #[test]
    fn should_build_decimal_literal_when_decimal_point_present() {
        // Arrange
        let input = "3.14";

        let pair = parse_string_to_rule(input, Rule::decimal_literal)
            .expect("Failed to parse decimal_literal")
            .next()
            .expect("Expected at least one pair for decimal_literal");

        // Act
        let result = build_decimal_literal(pair);

        // Assert
        assert_eq!(
            result.expect("Asserted successful build of decimal_literal"),
            Literal::Decimal(DecimalLiteral::new(3.14))
        );
    }

    #[test]
    fn should_build_negative_decimal_when_input_is_negative() {
        // Arrange
        let input = "-5.0";

        let pair = parse_string_to_rule(input, Rule::decimal_literal)
            .expect("Failed to parse decimal_literal")
            .next()
            .expect("Expected at least one pair for decimal_literal");

        // Act
        let result = build_decimal_literal(pair);

        // Assert
        assert_eq!(
            result.expect("Asserted successful build of decimal_literal"),
            Literal::Decimal(DecimalLiteral::new(-5.0))
        );
    }
}
