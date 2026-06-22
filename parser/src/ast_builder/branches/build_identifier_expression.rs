use crate::{into_ast_span::IntoAstSpan, rule_parser::Rule};
use ast::Identifier;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed identifier expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// - identifier: "myVariable"
/// ```
pub fn build_identifier_expression(
    pair: Pair<Rule>,
) -> Result<Identifier, BuildIdentifierExpressionError> {
    use BuildIdentifierExpressionError::*;
    use Rule::identifier;

    let rule = pair.as_rule();

    if rule != identifier {
        return Err(RuleIsNotAnIdentifier(rule));
    }

    let id = pair.to_string();
    let span = pair.as_span().into_ast_span();

    Ok(Identifier::new(id, span))
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildIdentifierExpressionError {
    /// The first rule is not an identifier.
    #[error("Expected an identifier expression, but found rule: {0:?}")]
    RuleIsNotAnIdentifier(Rule),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::Span;

    #[test]
    fn test_build_identifier_expression() {
        // Arrange
        let input = r"myVariable";

        let rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Failed to parse string to rule")
            .next()
            .expect("Expected at least one rule");

        // Act
        let identifier = build_identifier_expression(rule);

        // Assert
        assert!(identifier.is_ok());
        let identifier = identifier.unwrap();
        assert_eq!(identifier.id, "myVariable");
        assert_ne!(identifier.span, Span::DUMMY);
    }
}
