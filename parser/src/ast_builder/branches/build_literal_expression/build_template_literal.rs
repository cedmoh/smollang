use crate::rule_parser::Rule;
use ast::TemplateLiteral;
use pest::iterators::Pair;
use thiserror::Error;

/// Builds a template literal from a pest rule.
///
/// ```pest
/// template_literal > string_text: "Hello ${name}!"
/// ```
pub fn build_template_literal(
    pair: Pair<Rule>,
) -> Result<TemplateLiteral, BuildTemplateLiteralError> {
    use BuildTemplateLiteralError::*;
    use Rule::template_literal;

    let rule = pair.as_rule();

    if rule != template_literal {
        return Err(UnexpectedInnerLiteral(rule));
    };

    let string_text_pair = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::string_text)
        .ok_or(EmptyTemplateLiteral)?;

    Ok(TemplateLiteral::new(string_text_pair.as_str().to_string()))
}

#[derive(Debug, Error)]
pub enum BuildTemplateLiteralError {
    #[error("Expected a template_literal, but found {0:?}")]
    UnexpectedInnerLiteral(Rule),

    #[error("The template literal is empty.")]
    EmptyTemplateLiteral,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};

    #[test]
    fn should_build_template_literal() {
        // Arrange
        let input = r#"$'Hello {name}!'"#;

        let pair = parse_string_to_rule(input, Rule::template_literal)
            .expect("Failed to parse template_literal")
            .next()
            .expect("Expected at least one pair for template_literal");

        // Act
        let result = build_template_literal(pair);

        // Assert
        assert!(result.is_ok());
        assert_eq!(
            result.expect("Asserted successful build of template_literal"),
            TemplateLiteral::new("Hello {name}!".to_string())
        );
    }
}
