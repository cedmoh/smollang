use crate::rule_parser::Rule;
use ast::Identifier;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed identifier expression into an AST
/// representation.
/// ```
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

    Ok(Identifier::new(id))
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
        assert_eq!(identifier, Ok(Identifier::new("myVariable".to_string())));
    }
}
