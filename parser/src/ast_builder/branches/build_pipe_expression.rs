use crate::rule_parser::Rule;
use ast::Pipe;
use pest::iterators::Pair;
use thiserror::Error;

use super::utils::match_rule_to_expression_builder;

/// Converts the pest rules of a parsed pipe expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// pipe_expression
///   - pipe_first_arm > block: "()"
///   - pipe_arm > identifier: "t"
///   - ... (more pipe_arm rules)
/// ```
pub fn build_pipe_expression(
    pair: Pair<Rule>,
) -> Result<Pipe, BuildPipeExpressionError> {
    use BuildPipeExpressionError::*;
    use Rule::{pipe_arm, pipe_expression, pipe_first_arm};

    let rule = pair.as_rule();

    if rule != pipe_expression {
        return Err(RuleIsNotAPipeExpression(rule));
    };

    let mut inner = pair.into_inner();

    // Extract the first arm
    let first_arm_pair = inner.next().ok_or(MissingFirstArm)?;

    if first_arm_pair.as_rule() != pipe_first_arm {
        return Err(InvalidFirstArmRule(first_arm_pair.as_rule()));
    }

    // The pipe_first_arm contains an expression
    let first_expression_pair =
        first_arm_pair.into_inner().next().ok_or(EmptyFirstArm)?;

    let first_expression =
        match_rule_to_expression_builder(first_expression_pair)
            .map_err(|e| BuildExpressionError(e.to_string()))?;

    // Extract the second arm (at least one must exist)
    let second_arm_pair = inner.next().ok_or(MissingSecondArm)?;

    if second_arm_pair.as_rule() != pipe_arm {
        return Err(InvalidPipeArmRule(second_arm_pair.as_rule()));
    }

    // The pipe_arm contains an expression
    let second_expression_pair =
        second_arm_pair.into_inner().next().ok_or(EmptyPipeArm)?;

    let second_expression =
        match_rule_to_expression_builder(second_expression_pair)
            .map_err(|e| BuildExpressionError(e.to_string()))?;

    // Build the pipe using the builder pattern
    let mut pipe_builder = Pipe::builder(first_expression, second_expression);

    // Process any remaining pipe arms
    for arm_pair in inner {
        if arm_pair.as_rule() != pipe_arm {
            return Err(InvalidPipeArmRule(arm_pair.as_rule()));
        }

        let expression_pair =
            arm_pair.into_inner().next().ok_or(EmptyPipeArm)?;

        let expression = match_rule_to_expression_builder(expression_pair)
            .map_err(|e| BuildExpressionError(e.to_string()))?;

        pipe_builder.add_arm(expression);
    }

    Ok(pipe_builder.build())
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildPipeExpressionError {
    /// The first rule is not a pipe expression.
    #[error("Expected a pipe expression, but found rule: {0:?}")]
    RuleIsNotAPipeExpression(Rule),

    /// The first arm is missing.
    #[error("Missing first arm in pipe expression")]
    MissingFirstArm,

    /// The first arm rule is invalid.
    #[error("Expected pipe_first_arm, but found rule: {0:?}")]
    InvalidFirstArmRule(Rule),

    /// The first arm is empty.
    #[error("First arm in pipe expression is empty")]
    EmptyFirstArm,

    /// The second arm is missing (pipe requires at least two expressions).
    #[error(
        "Missing second arm in pipe expression (at least two arms required)"
    )]
    MissingSecondArm,

    /// A pipe arm rule is invalid.
    #[error("Expected pipe_arm, but found rule: {0:?}")]
    InvalidPipeArmRule(Rule),

    /// A pipe arm is empty.
    #[error("Pipe arm is empty")]
    EmptyPipeArm,

    /// Failed to build an expression from a pipe arm.
    #[error("Failed to build expression: {0}")]
    BuildExpressionError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::{Expression, Identifier};

    #[test]
    fn should_build_pipe_expression_with_two_arms_when_given_simple_identifiers()
     {
        // Arrange
        let input = r#"first |> second"#;

        let pipe_rule = parse_string_to_rule(input, Rule::pipe_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a pipe expression.");

        // Act
        let pipe_expression = build_pipe_expression(pipe_rule);

        // Assert
        let expected = Pipe::builder(
            Identifier::synthetic("first".to_string()).into(),
            Identifier::synthetic("second".to_string()).into(),
        )
        .build();

        assert_eq!(pipe_expression, Ok(expected));
    }

    #[test]
    fn should_build_pipe_expression_with_three_arms_when_given_multiple_pipes()
    {
        // Arrange
        let input = r#"first |> second |> third"#;

        let pipe_rule = parse_string_to_rule(input, Rule::pipe_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a pipe expression.");

        // Act
        let pipe_expression = build_pipe_expression(pipe_rule);

        // Assert
        let expected = Pipe::builder(
            Identifier::synthetic("first".to_string()).into(),
            Identifier::synthetic("second".to_string()).into(),
        )
        .with_arm(Identifier::synthetic("third".to_string()).into())
        .build();

        assert_eq!(pipe_expression, Ok(expected));
    }

    #[test]
    fn should_build_pipe_expression_with_multiple_arms_when_given_many_pipes() {
        // Arrange
        let input = r#"a |> b |> c |> d |> e"#;

        let pipe_rule = parse_string_to_rule(input, Rule::pipe_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a pipe expression.");

        // Act
        let pipe_expression = build_pipe_expression(pipe_rule);

        // Assert
        assert!(pipe_expression.is_ok());
        let pipe = pipe_expression.unwrap();
        assert_eq!(pipe.arms.arms.len(), 5);

        // Verify each arm
        let arm_names = vec!["a", "b", "c", "d", "e"];
        for (i, expected_name) in arm_names.iter().enumerate() {
            match &pipe.arms.arms[i].expression {
                Expression::Identifier(id) => {
                    assert_eq!(id.id, *expected_name);
                }
                _ => panic!("Expected identifier in arm {}", i),
            }
        }
    }

    #[test]
    fn should_build_pipe_expression_with_operations() {
        // Arrange
        // Operations are allowed in pipe arms (except first arm)
        let input = r#"a |> add(b, c)"#;

        let pipe_rule = parse_string_to_rule(input, Rule::pipe_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a pipe expression.");

        // Act
        let pipe_expression = build_pipe_expression(pipe_rule);

        // Assert
        assert!(pipe_expression.is_ok());
        let pipe = pipe_expression.unwrap();
        assert_eq!(pipe.arms.arms.len(), 2);

        // First arm should be an identifier
        assert!(matches!(
            pipe.arms.arms[0].expression,
            Expression::Identifier(_)
        ));

        // Second arm should be a function call
        assert!(matches!(
            pipe.arms.arms[1].expression,
            Expression::FunctionCall(_)
        ));
    }

    #[test]
    fn should_build_pipe_expression_with_block_expressions() {
        // Arrange
        let input = r#"(a) |> (b)"#;

        let pipe_rule = parse_string_to_rule(input, Rule::pipe_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a pipe expression.");

        // Act
        let pipe_expression = build_pipe_expression(pipe_rule);

        // Assert
        assert!(pipe_expression.is_ok());
        let pipe = pipe_expression.unwrap();
        assert_eq!(pipe.arms.arms.len(), 2);

        // Both arms should be blocks
        assert!(matches!(pipe.arms.arms[0].expression, Expression::Block(_)));
        assert!(matches!(pipe.arms.arms[1].expression, Expression::Block(_)));
    }

    #[test]
    fn should_return_error_when_rule_is_not_a_pipe_expression() {
        // Arrange
        let input = r#"identifier"#;

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier.");

        // Act
        let result = build_pipe_expression(identifier_rule);

        // Assert
        assert!(matches!(
            result,
            Err(BuildPipeExpressionError::RuleIsNotAPipeExpression(_))
        ));
    }
}
