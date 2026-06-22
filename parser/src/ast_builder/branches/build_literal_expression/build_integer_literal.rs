use crate::{into_ast_span::IntoAstSpan, rule_parser::Rule};
use ast::{IntegerLiteral, Literal};
use pest::iterators::Pair;
use thiserror::Error;

/// Builds an [`IntegerLiteral`] from a pest rule.
///
/// ```pest
/// integer_literal: "42"
/// ```
pub fn build_integer_literal(
    pair: Pair<Rule>,
) -> Result<Literal, BuildIntegerLiteralError> {
    use BuildIntegerLiteralError::*;
    use Rule::integer_literal;

    let rule = pair.as_rule();

    if rule != integer_literal {
        return Err(UnexpectedRule(rule));
    }

    let span = pair.as_span().into_ast_span();

    let text = pair.as_str().trim().replace('_', "");

    let value: i32 =
        text.parse().map_err(|_| ParseIntegerFailed(text.clone()))?;

    Ok(IntegerLiteral::new(value, span).into())
}

#[derive(Debug, Error)]
pub enum BuildIntegerLiteralError {
    #[error("Expected an integer_literal, but found {0:?}")]
    UnexpectedRule(Rule),

    #[error("Failed to parse '{0}' as an integer number")]
    ParseIntegerFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};
    use ast::Span;

    #[test]
    fn should_build_integer_literal_when_integer_present() {
        // Arrange
        let input = "42";

        let pair = parse_string_to_rule(input, Rule::integer_literal)
            .expect("Failed to parse integer_literal")
            .next()
            .expect("Expected at least one pair for integer_literal");

        // Act
        let result = build_integer_literal(pair);

        // Assert
        let result = result.expect("Asserted successful build of integer_literal");
        match result {
            Literal::Integer(value) => {
                assert_eq!(value.value, 42);
                assert_ne!(value.span, Span::DUMMY);
            }
            _ => panic!("Expected integer literal"),
        }
    }

    #[test]
    fn should_build_negative_integer_when_input_is_negative() {
        // Arrange
        let input = "-5";

        let pair = parse_string_to_rule(input, Rule::integer_literal)
            .expect("Failed to parse integer_literal")
            .next()
            .expect("Expected at least one pair for integer_literal");

        // Act
        let result = build_integer_literal(pair);

        // Assert
        let result = result.expect("Asserted successful build of integer_literal");
        match result {
            Literal::Integer(value) => {
                assert_eq!(value.value, -5);
                assert_ne!(value.span, Span::DUMMY);
            }
            _ => panic!("Expected integer literal"),
        }
    }
}
