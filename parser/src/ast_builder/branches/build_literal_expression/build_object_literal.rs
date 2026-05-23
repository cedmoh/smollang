use crate::rule_parser::Rule;
use ast::{Literal, ObjectLiteral, ObjectProperty};
use pest::iterators::Pair;
use thiserror::Error;

use crate::ast_builder::{
    build_literal_expression, match_rule_to_expression_builder,
};

/// Builds an object literal from a pest rule.
///
/// ```pest
/// object_literal > object_entries
///     - object_entry
///       - object_key > identifier: "name"
///       - object_value > expression > literal > string_literal > string_text: "Alice"
///     - object_entry
///       - object_key > identifier: "age"
///       - object_value > expression > literal > integer_literal: "30"
///     - object_entry
///       - object_key > identifier: "isStudent"
///       - object_value > expression > literal > boolean_literal: "false"
///     - object_entry
///       - object_key > identifier: "salary"
///       - object_value > expression > call
///         - callee > identifier: "getSalary"
///         - call_arguments: "()"
///     - object_entry
///       - object_key > identifier: "study"
///       - object_value > expression > function_declaration
///         - function_arguments: "||"
///         - function_body > expression > call
///           - callee > identifier: "print"
///           - call_arguments > literal > string_literal > string_text: "Studying..."
/// ```
pub fn build_object_literal(
    pair: Pair<Rule>,
) -> Result<ObjectLiteral, BuildObjectLiteralError> {
    use BuildObjectLiteralError::*;
    use Rule::{
        object_entries, object_entry, object_key, object_literal, object_value,
    };

    let rule = pair.as_rule();

    if rule != object_literal {
        return Err(UnexpectedRule(rule));
    }

    let entries_pair = pair
        .into_inner()
        .find(|p| p.as_rule() == object_entries)
        .ok_or(MissingObjectEntries)?;

    let mut object_builder = ObjectLiteral::builder();

    for entry_pair in entries_pair.into_inner() {
        if entry_pair.as_rule() != object_entry {
            return Err(UnexpectedObjectEntryRule(entry_pair.as_rule()));
        }

        let mut entry_inner = entry_pair.into_inner();

        let key_pair = entry_inner.next().ok_or(MissingObjectKey)?;
        let value_pair = entry_inner.next().ok_or(MissingObjectValue)?;

        if key_pair.as_rule() != object_key {
            return Err(UnexpectedObjectKeyRule(key_pair.as_rule()));
        }

        if value_pair.as_rule() != object_value {
            return Err(UnexpectedObjectValueRule(value_pair.as_rule()));
        }

        let key = build_object_key(key_pair)?;

        let value_expression_pair = value_pair
            .into_inner()
            .next()
            .ok_or(MissingObjectValueExpression)?;

        let value_expression = match_rule_to_expression_builder(
            value_expression_pair,
        )
        .map_err(|error| BuildObjectValueExpressionError(error.to_string()))?;

        object_builder
            .add_property(ObjectProperty::KeyValue(key, value_expression));
    }

    Ok(object_builder.build())
}

fn build_object_key(
    pair: Pair<Rule>,
) -> Result<String, BuildObjectLiteralError> {
    use BuildObjectLiteralError::*;
    use Rule::{
        dynamic_key_expression, identifier, literal as literal_rule, object_key,
    };

    if pair.as_rule() != object_key {
        return Err(UnexpectedObjectKeyRule(pair.as_rule()));
    }

    let inner_key = pair.into_inner().next().ok_or(EmptyObjectKey)?;

    match inner_key.as_rule() {
        identifier => Ok(inner_key.as_str().to_string()),
        literal_rule => {
            let literal_value =
                build_literal_expression(inner_key).map_err(|error| {
                    BuildObjectKeyLiteralError(error.to_string())
                })?;

            literal_to_object_key(literal_value)
        }
        dynamic_key_expression => Err(UnsupportedDynamicObjectKey),
        rule => Err(UnexpectedObjectKeyInnerRule(rule)),
    }
}

fn literal_to_object_key(
    literal: Literal,
) -> Result<String, BuildObjectLiteralError> {
    use BuildObjectLiteralError::*;

    match literal {
        Literal::Nil => Ok("nil".to_string()),
        Literal::Boolean(value) => Ok(value.value.to_string()),
        Literal::String(value) => Ok(value.value),
        Literal::Integer(value) => Ok(value.value.to_string()),
        Literal::Decimal(value) => Ok(value.value.to_string()),
        Literal::Hexadecimal(value) => Ok(value.value.to_string()),
        Literal::Binary(value) => Ok(value.value.to_string()),
        Literal::Octal(value) => Ok(value.value.to_string()),
        Literal::Array(_) => {
            Err(UnsupportedLiteralObjectKey("array".to_string()))
        }
        Literal::Object(_) => {
            Err(UnsupportedLiteralObjectKey("object".to_string()))
        }
    }
}

#[derive(Debug, Error)]
pub enum BuildObjectLiteralError {
    #[error("Expected an object_literal, but found {0:?}")]
    UnexpectedRule(Rule),

    #[error("Expected object_literal to contain object_entries")]
    MissingObjectEntries,

    #[error("Expected an object_entry, but found {0:?}")]
    UnexpectedObjectEntryRule(Rule),

    #[error("Expected object_entry to contain object_key")]
    MissingObjectKey,

    #[error("Expected object_entry to contain object_value")]
    MissingObjectValue,

    #[error("Expected object_key rule, but found {0:?}")]
    UnexpectedObjectKeyRule(Rule),

    #[error("Expected object_value rule, but found {0:?}")]
    UnexpectedObjectValueRule(Rule),

    #[error("Expected object_key to contain an inner key")]
    EmptyObjectKey,

    #[error("Unexpected rule found for object key: {0:?}")]
    UnexpectedObjectKeyInnerRule(Rule),

    #[error("Failed to build literal object key: {0}")]
    BuildObjectKeyLiteralError(String),

    #[error("Dynamic object keys are not supported yet")]
    UnsupportedDynamicObjectKey,

    #[error("Unsupported literal object key kind: {0}")]
    UnsupportedLiteralObjectKey(String),

    #[error("Expected object_value to contain an expression")]
    MissingObjectValueExpression,

    #[error("Failed to build object value expression: {0}")]
    BuildObjectValueExpressionError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_string_to_rule, rule_parser::Rule};
    use ast::{
        BooleanLiteral, Identifier, IntegerLiteral, Literal, StringLiteral,
    };

    #[test]
    fn should_build_object_literal_with_identifier_keys() {
        // Arrange
        let input = "{ name 'Alice', age 30, active true, user profile }";

        let pair = parse_string_to_rule(input, Rule::object_literal)
            .expect("Expected input to be parsed to object_literal")
            .next()
            .expect("Expected at least one pair for object_literal");

        // Act
        let result = build_object_literal(pair);

        // Assert
        let expected = ObjectLiteral::builder()
            .with_property(ObjectProperty::KeyValue(
                "name".to_string(),
                Literal::String(StringLiteral::new("Alice".to_string())).into(),
            ))
            .with_property(ObjectProperty::KeyValue(
                "age".to_string(),
                Literal::Integer(IntegerLiteral::new(30)).into(),
            ))
            .with_property(ObjectProperty::KeyValue(
                "active".to_string(),
                Literal::Boolean(BooleanLiteral { value: true }).into(),
            ))
            .with_property(ObjectProperty::KeyValue(
                "user".to_string(),
                Identifier::new("profile".to_string()).into(),
            ))
            .build();

        assert_eq!(
            result.expect("Expected object literal to be built successfully"),
            expected
        );
    }

    #[test]
    fn should_build_object_literal_with_literal_keys() {
        // Arrange
        let input = "{ 'name' 'Alice', 1 true }";

        let pair = parse_string_to_rule(input, Rule::object_literal)
            .expect("Expected input to be parsed to object_literal")
            .next()
            .expect("Expected at least one pair for object_literal");

        // Act
        let result = build_object_literal(pair);

        // Assert
        let expected = ObjectLiteral::builder()
            .with_property(ObjectProperty::KeyValue(
                "name".to_string(),
                Literal::String(StringLiteral::new("Alice".to_string())).into(),
            ))
            .with_property(ObjectProperty::KeyValue(
                "1".to_string(),
                Literal::Boolean(BooleanLiteral { value: true }).into(),
            ))
            .build();

        assert_eq!(
            result.expect("Expected object literal with literal keys to build"),
            expected
        );
    }

    #[test]
    fn should_build_empty_object_literal_when_no_entries_present() {
        // Arrange
        let input = "{}";

        let pair = parse_string_to_rule(input, Rule::object_literal)
            .expect("Expected input to be parsed to object_literal")
            .next()
            .expect("Expected at least one pair for object_literal");

        // Act
        let result = build_object_literal(pair);

        // Assert
        assert_eq!(
            result.expect("Expected empty object literal to be built"),
            ObjectLiteral::builder().build()
        );
    }

    #[test]
    fn should_return_error_when_rule_is_not_object_literal() {
        // Arrange
        let input = "42";

        let pair = parse_string_to_rule(input, Rule::integer_literal)
            .expect("Expected input to be parsed to integer_literal")
            .next()
            .expect("Expected at least one pair for integer_literal");

        // Act
        let result = build_object_literal(pair);

        // Assert
        assert!(matches!(
            result,
            Err(BuildObjectLiteralError::UnexpectedRule(
                Rule::integer_literal
            ))
        ));
    }
}
