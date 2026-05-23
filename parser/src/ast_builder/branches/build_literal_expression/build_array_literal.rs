use crate::rule_parser::Rule;
use ast::ArrayLiteral;
use pest::iterators::Pair;
use thiserror::Error;

use crate::ast_builder::match_rule_to_expression_builder;

/// Builds an array literal from a pest rule.
///
/// ```pest
/// array_literal > array_entries
///     - array_entry > expression > literal > integer_literal: "1"
///     - array_entry > expression > identifier: "one"
///     - array_entry > expression > literal > boolean_literal: "true"
///     - array_entry > expression > literal > string_literal > string_text: "hello"
///     - array_entry > expression > literal > array_literal > array_entries: ""
///     - array_entry > expression > call
///         - callee > identifier: "print"
///         - call_arguments
///             - literal > string_literal > string_text: "hola!"
/// ```
pub fn build_array_literal(
    pair: Pair<Rule>,
) -> Result<ArrayLiteral, BuildArrayLiteralError> {
    use BuildArrayLiteralError::*;
    use Rule::{array_entries, array_entry, array_literal};

    let rule = pair.as_rule();

    if rule != array_literal {
        return Err(UnexpectedRule(rule));
    }

    let entries_pair = pair
        .into_inner()
        .find(|p| p.as_rule() == array_entries)
        .ok_or(MissingArrayEntries)?;

    let mut array_builder = ArrayLiteral::builder();

    for entry_pair in entries_pair.into_inner() {
        if entry_pair.as_rule() != array_entry {
            return Err(UnexpectedArrayEntryRule(entry_pair.as_rule()));
        }

        let expression_pair = entry_pair
            .into_inner()
            .next()
            .ok_or(MissingArrayEntryExpression)?;

        let expression = match_rule_to_expression_builder(expression_pair)
            .map_err(|error| {
                BuildArrayEntryExpressionError(error.to_string())
            })?;

        array_builder.add_element(expression);
    }

    Ok(array_builder.build())
}

#[derive(Debug, Error)]
pub enum BuildArrayLiteralError {
    #[error("Expected an array_literal, but found {0:?}")]
    UnexpectedRule(Rule),

    #[error("Expected array_literal to contain array_entries")]
    MissingArrayEntries,

    #[error("Expected an array_entry, but found {0:?}")]
    UnexpectedArrayEntryRule(Rule),

    #[error("Expected array_entry to contain an expression")]
    MissingArrayEntryExpression,

    #[error("Failed to build an expression for array_entry: {0}")]
    BuildArrayEntryExpressionError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};
    use ast::{
        BooleanLiteral, Identifier, IntegerLiteral, Literal, StringLiteral,
    };

    #[test]
    fn should_build_array_literal_with_mixed_elements() {
        // Arrange
        let input = "[1, two, true, 'hello']";

        let pair = parse_string_to_rule(input, Rule::array_literal)
            .expect("Expected input to be parsed to array_literal")
            .next()
            .expect("Expected at least one pair for array_literal");

        // Act
        let result = build_array_literal(pair);

        // Assert
        let expected = ArrayLiteral::builder()
            .with_element(Literal::Integer(IntegerLiteral::new(1)).into())
            .with_element(Identifier::new("two".to_string()).into())
            .with_element(
                Literal::Boolean(BooleanLiteral { value: true }).into(),
            )
            .with_element(
                Literal::String(StringLiteral::new("hello".to_string())).into(),
            )
            .build();

        assert_eq!(
            result.expect("Expected array literal to be built successfully"),
            expected
        );
    }

    #[test]
    fn should_build_empty_array_literal_when_no_entries_present() {
        // Arrange
        let input = "[]";

        let pair = parse_string_to_rule(input, Rule::array_literal)
            .expect("Expected input to be parsed to array_literal")
            .next()
            .expect("Expected at least one pair for array_literal");

        // Act
        let result = build_array_literal(pair);

        // Assert
        assert_eq!(
            result.expect(
                "Expected empty array literal to be built successfully"
            ),
            ArrayLiteral::builder().build()
        );
    }

    #[test]
    fn should_return_error_when_rule_is_not_array_literal() {
        // Arrange
        let input = "true";

        let pair = parse_string_to_rule(input, Rule::boolean_literal)
            .expect("Expected input to be parsed to boolean_literal")
            .next()
            .expect("Expected at least one pair for boolean_literal");

        // Act
        let result = build_array_literal(pair);

        // Assert
        assert!(matches!(
            result,
            Err(BuildArrayLiteralError::UnexpectedRule(
                Rule::boolean_literal
            ))
        ));
    }
}
