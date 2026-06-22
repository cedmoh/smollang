use crate::{
    ast_builder::{
        BuildAstExpressionError, build_ast_expression,
        match_rule_to_expression_builder,
    },
    into_ast_span::IntoAstSpan,
    rule_parser::Rule,
};
use ast::Then;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed then expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// - then_expression
///   - then_condition > literal > ...
///   - then_body > expression > ...
///   - then_else_body (optional) > expression > ...
/// ```
pub fn build_then_expression(
    pair: Pair<Rule>,
) -> Result<Then, BuildThenExpressionError> {
    pub use BuildThenExpressionError::*;
    pub use Rule::{
        then_body, then_check_symbol, then_condition, then_else_body,
        then_else_symbol, then_expression,
    };

    let rule = pair.as_rule();

    if rule != then_expression {
        return Err(RuleIsNotAThenExpression(rule));
    };

    let span = pair.as_span().into_ast_span();

    let mut inner = pair.into_inner();

    // Extract the condition
    let condition_pair = inner.next().ok_or(MissingCondition)?;

    if condition_pair.as_rule() != then_condition {
        return Err(InvalidConditionRule(condition_pair.as_rule()));
    }

    let rule = condition_pair.as_rule();

    if rule != then_condition {
        return Err(InvalidConditionExpressionRule(rule));
    }

    // Extract the inner expression type from then_condition
    let inner_expression =
        condition_pair.into_inner().next().ok_or(EmptyCondition)?;

    // Match on the inner expression type and build it using the same logic
    // as build_ast_expression, but adapted for then_condition types
    let condition_expression =
        match_rule_to_expression_builder(inner_expression)?;

    // Skip the then_check_symbol
    let next_pair = inner.next().ok_or(MissingThenBody)?;

    let then_body_pair = if next_pair.as_rule() == then_check_symbol {
        // Skip the symbol and get the actual then_body
        inner.next().ok_or(MissingThenBody)?
    } else {
        // This is already the then_body
        next_pair
    };

    if then_body_pair.as_rule() != then_body {
        return Err(InvalidThenBodyRule(then_body_pair.as_rule()));
    }

    // The then_body rule contains an expression
    let then_body_expression_pair =
        then_body_pair.into_inner().next().ok_or(EmptyThenBody)?;

    let then_body_expression = build_ast_expression(then_body_expression_pair)?;

    // Build the Then using the builder pattern
    let mut then_builder =
        Then::builder(condition_expression, then_body_expression);

    // Check for optional else_body
    // Skip the then_else_symbol if present
    if let Some(next_pair) = inner.next() {
        let else_body_pair = if next_pair.as_rule() == then_else_symbol {
            // Skip the symbol and get the actual else_body
            inner.next()
        } else if next_pair.as_rule() == then_else_body {
            // This is already the else_body
            Some(next_pair)
        } else {
            None
        };

        if let Some(else_pair) = else_body_pair {
            if else_pair.as_rule() != then_else_body {
                return Err(InvalidElseBodyRule(else_pair.as_rule()));
            }

            // The then_else_body rule contains an expression
            let else_body_expression_pair =
                else_pair.into_inner().next().ok_or(EmptyElseBody)?;

            let else_body_expression =
                build_ast_expression(else_body_expression_pair)?;

            then_builder.add_else_body(else_body_expression);
        }
    }

    Ok(then_builder.with_span(span).build())
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildThenExpressionError {
    /// The first rule is not a then expression.
    #[error("Expected a then expression, but found rule: {0:?}")]
    RuleIsNotAThenExpression(Rule),

    /// The condition rule is missing from the then expression.
    #[error("Missing condition in then expression")]
    MissingCondition,

    /// The condition rule is not a then_condition.
    #[error("Expected a then_condition rule, but found rule: {0:?}")]
    InvalidConditionRule(Rule),

    /// The condition expression is empty.
    #[error("The condition expression is empty")]
    EmptyCondition,

    /// The condition expression has an invalid rule type.
    #[error("Invalid condition expression rule: {0:?}")]
    InvalidConditionExpressionRule(Rule),

    /// The then body rule is missing from the then expression.
    #[error("Missing then body in then expression")]
    MissingThenBody,

    /// The then body rule is not a then_body.
    #[error("Expected a then_body rule, but found rule: {0:?}")]
    InvalidThenBodyRule(Rule),

    /// The then body expression is empty.
    #[error("The then body expression is empty")]
    EmptyThenBody,

    /// The else body rule is not a then_else_body.
    #[error("Expected a then_else_body rule, but found rule: {0:?}")]
    InvalidElseBodyRule(Rule),

    /// The else body expression is empty.
    #[error("The else body expression is empty")]
    EmptyElseBody,

    /// An error occurred while building an expression within the then.
    #[error("An error occurred while building an expression: {0}")]
    BuildAstExpressionError(#[from] BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::{Expression, Span};

    #[test]
    fn should_build_then_expression_without_else_body() {
        // Arrange
        let input = r#"condition then thenBody"#;

        let then_rule = parse_string_to_rule(input, Rule::then_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a then expression.");

        // Act
        let then_expression = build_then_expression(then_rule);

        // Assert
        assert!(then_expression.is_ok());
        let then_expression = then_expression.unwrap();
        assert!(matches!(then_expression.condition.as_ref(), Expression::Identifier(_)));
        assert!(matches!(then_expression.then_body.as_ref(), Expression::Identifier(_)));
        assert!(then_expression.else_body.is_none());
        assert_ne!(then_expression.span, Span::DUMMY);
    }

    #[test]
    fn should_build_then_expression_with_else_body() {
        // Arrange
        let input = r#"condition then thenBody else elseBody"#;

        let then_rule = parse_string_to_rule(input, Rule::then_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a then expression.");

        // Act
        let then_expression = build_then_expression(then_rule);

        // Assert
        assert!(then_expression.is_ok());
        let then_expression = then_expression.unwrap();
        assert!(matches!(then_expression.condition.as_ref(), Expression::Identifier(_)));
        assert!(matches!(then_expression.then_body.as_ref(), Expression::Identifier(_)));
        assert!(then_expression.else_body.is_some());
        assert_ne!(then_expression.span, Span::DUMMY);
    }

    #[test]
    fn should_build_then_expression_with_parenthesized_condition() {
        // Arrange
        let input = r#"(condition) then thenBody else elseBody"#;

        let then_rule = parse_string_to_rule(input, Rule::then_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a then expression.");

        // Act
        let then_expression = build_then_expression(then_rule);

        // Assert
        // Parenthesized expressions are parsed as blocks containing the
        // expression
        assert!(then_expression.is_ok());
        let then_expr = then_expression.unwrap();
        assert!(matches!(then_expr.condition.as_ref(), Expression::Block(_)));
        assert!(matches!(
            then_expr.then_body.as_ref(),
            Expression::Identifier(_)
        ));
        assert!(then_expr.else_body.is_some());
    }

    #[test]
    fn should_build_then_expression_with_nested_then() {
        // Arrange
        // Nested then expressions require parentheses in the then_body
        // because then_expression is not allowed in then_first_expression
        let input = r#"outerCondition then innerCondition else outerElse"#;

        let then_rule = parse_string_to_rule(input, Rule::then_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a then expression.");

        // Act
        let then_expression = build_then_expression(then_rule);

        // Assert
        // Just verify it doesn't error
        assert!(then_expression.is_ok());

        // Verify the structure
        let then_expr = then_expression.unwrap();
        assert!(matches!(
            then_expr.condition.as_ref(),
            Expression::Identifier(_)
        ));
        assert!(matches!(
            then_expr.then_body.as_ref(),
            Expression::Identifier(_)
        ));
        assert!(then_expr.else_body.is_some());
    }

    #[test]
    fn should_return_error_when_rule_is_not_then_expression() {
        // Arrange
        let input = r#"myIdentifier"#;

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier.");

        // Act
        let then_expression = build_then_expression(identifier_rule);

        // Assert
        assert!(matches!(
            then_expression,
            Err(BuildThenExpressionError::RuleIsNotAThenExpression(_))
        ));
    }
}
