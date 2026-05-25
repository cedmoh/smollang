use crate::rule_parser::Rule;
use ast::StringLiteral;
use pest::iterators::Pair;
use thiserror::Error;

/// Builds a string literal from a pest rule.
///
/// ```pest
/// string_literal > string_text: "Hello World!"
/// ```
pub fn build_string_literal(
    pair: Pair<Rule>,
) -> Result<StringLiteral, BuildStringLiteralError> {
    use BuildStringLiteralError::*;
    use Rule::string_literal;

    let rule = pair.as_rule();

    if rule != string_literal {
        return Err(UnexpectedInnerLiteral(rule));
    };

    let string_text_pair = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::string_text)
        .ok_or(EmptyStringLiteral)?;

    Ok(StringLiteral::new(string_text_pair.as_str().to_string()))
}

#[derive(Debug, Error)]
pub enum BuildStringLiteralError {
    #[error("Expected a string_literal, but found {0:?}")]
    UnexpectedInnerLiteral(Rule),

    #[error("The string literal is empty.")]
    EmptyStringLiteral,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};

    #[test]
    fn should_build_string_literal() {
        // Arrange
        let input = r#"'Hello World!'"#;

        let pair = parse_string_to_rule(input, Rule::string_literal)
            .expect("Failed to parse string_literal")
            .next()
            .expect("Expected at least one pair for string_literal");

        // Act
        let result = build_string_literal(pair);

        // Assert
        assert!(result.is_ok());
        assert_eq!(
            result.expect("Asserted successful build of string_literal"),
            StringLiteral::new("Hello World!".to_string())
        );
    }
}
