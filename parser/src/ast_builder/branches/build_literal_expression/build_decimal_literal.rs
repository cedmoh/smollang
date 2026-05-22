use crate::rule_parser::Rule;
use ast::{DecimalLiteral, IntegerLiteral, Literal};
use pest::iterators::Pair;
use thiserror::Error;

/// Builds a decimal or integer literal from a pest rule.
///
/// If the text representation contains a decimal point, a [`DecimalLiteral`]
/// is returned. Otherwise, an [`IntegerLiteral`] is returned.
///
/// ```pest
/// decimal_literal: "3.14"
/// ```
/// ```pest
/// decimal_literal: "42"
/// ```
pub fn build_decimal_literal(pair: Pair<Rule>) -> Result<Literal, BuildDecimalLiteralError> {
    use BuildDecimalLiteralError::*;
    use Rule::decimal_literal;

    let rule = pair.as_rule();

    if rule != decimal_literal {
        return Err(UnexpectedRule(rule));
    }

    let text = pair.as_str().replace('_', "");

    if text.contains('.') {
        let value: f64 = text.parse().map_err(|_| ParseDecimalFailed(text.clone()))?;
        Ok(DecimalLiteral::new(value).into())
    } else {
        let value: i64 = text.parse().map_err(|_| ParseIntegerFailed(text.clone()))?;
        Ok(IntegerLiteral::new(value).into())
    }
}

#[derive(Debug, Error)]
pub enum BuildDecimalLiteralError {
    #[error("Expected a decimal_literal, but found {0:?}")]
    UnexpectedRule(Rule),

    #[error("Failed to parse '{0}' as a decimal number")]
    ParseDecimalFailed(String),

    #[error("Failed to parse '{0}' as an integer")]
    ParseIntegerFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};

    #[test]
    fn should_build_integer_literal_when_no_decimal_point() {
        // Arrange
        let input = "42";

        let pair = parse_string_to_rule(input, Rule::decimal_literal)
            .expect("Failed to parse decimal_literal")
            .next()
            .expect("Expected at least one pair for decimal_literal");

        // Act
        let result = build_decimal_literal(pair);

        // Assert
        assert_eq!(
            result.expect("Asserted successful build of decimal_literal"),
            Literal::Integer(IntegerLiteral::new(42))
        );
    }

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
    fn should_build_negative_integer_when_input_is_negative() {
        // Arrange
        let input = "-5";

        let pair = parse_string_to_rule(input, Rule::decimal_literal)
            .expect("Failed to parse decimal_literal")
            .next()
            .expect("Expected at least one pair for decimal_literal");

        // Act
        let result = build_decimal_literal(pair);

        // Assert
        assert_eq!(
            result.expect("Asserted successful build of decimal_literal"),
            Literal::Integer(IntegerLiteral::new(-5))
        );
    }
}
