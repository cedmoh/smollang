use crate::{
    ast_builder::{
        BuildAstExpressionError, BuildIdentifierExpressionError,
        build_ast_expression, build_identifier_expression,
    },
    rule_parser::Rule,
};
use ast::FunctionCall;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed call expression into an AST
/// representation.
///
/// # Examples
///
/// Call without arguments:
///
/// ```pest
/// - call
///   - callee > identifier: "hello"
///   - call_arguments: "()"
/// ```
///
/// Call with literal argument:
///
/// ```pest
/// call
/// - callee > identifier: "print"
/// - call_arguments > literal > string_literal > string_text: "Hello small world!"
/// ```
///
/// Call with arguments:
///
/// ```pest
/// - call
///   - callee > identifier: "print"
///   - call_arguments
///     - literal > string_literal > string_text: "hello"
///     - literal > decimal_literal: "10"
/// ```
pub fn build_call_expression(
    pair: Pair<Rule>,
) -> Result<FunctionCall, BuildCallExpressionError> {
    use Rule::call;

    let rule = pair.as_rule();

    if rule != call {
        return Err(BuildCallExpressionError::RuleIsNotACall(rule));
    };

    let mut inner = pair.into_inner();

    // Extract the callee (the function being called)
    let callee_pair = inner.next().expect("Expected call to have a callee");

    // The callee rule contains an identifier, so we need to extract it
    let callee_identifier = callee_pair
        .into_inner()
        .next()
        .expect("Expected callee to contain an identifier");

    // Build the callee as an identifier expression
    // TODO: In the future, we want to support more complex expressions as
    // the callee (e.g., member expressions)
    let callee_expression =
        build_identifier_expression(callee_identifier)?.into();

    let mut function_call_builder = FunctionCall::builder(callee_expression);

    if let Some(call_arguments_pair) = inner.next() {
        // Skip call_generics if present, process call_arguments
        let arguments_inner =
            if call_arguments_pair.as_rule() == Rule::call_generics {
                // If we have generics, get the next pair which should be
                // call_arguments
                inner.next()
            } else {
                // Otherwise, this is already call_arguments
                Some(call_arguments_pair)
            };

        if let Some(args_pair) = arguments_inner {
            // call_arguments contains individual expressions
            for expression_pair in args_pair.into_inner() {
                function_call_builder
                    .add_argument(build_ast_expression(expression_pair)?);
            }
        }
    }

    Ok(function_call_builder.build())
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildCallExpressionError {
    /// The first rule is not a call expression.
    #[error("Expected a call expression, but found rule: {0:?}")]
    RuleIsNotACall(Rule),

    /// An error occurred while building the callee identifier.
    #[error("An error occurred while building the callee identifier: {0}")]
    BuildIdentifierExpressionError(#[from] BuildIdentifierExpressionError),

    /// An error occurred while building an expression within the call.
    #[error(
        "An error occurred while building an expression within the call: {0}"
    )]
    BuildAstExpressionError(#[from] BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use ast::Identifier;

    use super::*;
    use crate::rule_parser::parse_string_to_rule;

    #[test]
    fn should_build_call_expression_without_arguments() {
        // Arrange
        let input = r#"hello()"#;

        let call_rule = parse_string_to_rule(input, Rule::call)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected at least one rule");

        // Act
        let call_expression = build_call_expression(call_rule);

        let expected_call_expression =
            FunctionCall::builder(Identifier::synthetic("hello".into()).into())
                .build();

        // Assert
        assert_eq!(call_expression, Ok(expected_call_expression));
    }

    #[test]
    fn should_build_call_expression_with_arguments() {
        // Arrange
        let input = r#"add(x, y)"#;

        let call_rule = parse_string_to_rule(input, Rule::call)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected at least one rule");

        // Act
        let call_expression = build_call_expression(call_rule);

        // Assert
        let function_call =
            FunctionCall::builder(Identifier::synthetic("add".into()).into())
                .with_argument(Identifier::synthetic("x".into()))
                .with_argument(Identifier::synthetic("y".into()))
                .build();

        assert_eq!(call_expression, Ok(function_call));
    }
}
