use crate::{
    ast_builder::{
        BuildAstExpressionError, BuildIdentifierExpressionError,
        build_ast_expression, build_identifier_expression,
    },
    rule_parser::Rule,
};
use ast::Assignment;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed assignment expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// assignment_expression
///   - assignment_left_hand_side > identifier > ...
///   - assignment_right_hand_side > expression > ...
/// ```
pub fn build_assignment_expression(
    pair: Pair<Rule>,
) -> Result<Assignment, BuildAssignmentExpressionError> {
    use BuildAssignmentExpressionError::*;
    use Rule::{
        assignment_expression, assignment_left_hand_side,
        assignment_right_hand_side,
    };

    let rule = pair.as_rule();

    if rule != assignment_expression {
        return Err(RuleIsNotAnAssignment(rule));
    };

    let mut inner = pair.into_inner();

    let left_hand_side_pair = inner.next().ok_or(MissingLeftHandSide)?;

    if left_hand_side_pair.as_rule() != assignment_left_hand_side {
        return Err(InvalidLeftHandSideRule(left_hand_side_pair.as_rule()));
    }

    let identifier_pair = left_hand_side_pair
        .into_inner()
        .next()
        .ok_or(EmptyLeftHandSide)?;

    let identifier = build_identifier_expression(identifier_pair)?;

    let right_hand_side_pair = inner.next().ok_or(MissingRightHandSide)?;

    if right_hand_side_pair.as_rule() != assignment_right_hand_side {
        return Err(InvalidRightHandSideRule(right_hand_side_pair.as_rule()));
    }

    let value_pair = right_hand_side_pair
        .into_inner()
        .next()
        .ok_or(EmptyRightHandSide)?;

    let value = build_ast_expression(value_pair)?;

    Ok(Assignment::synthetic(identifier, Box::new(value)))
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildAssignmentExpressionError {
    /// The first rule is not an assignment expression.
    #[error("Expected an assignment expression, but found rule: {0:?}")]
    RuleIsNotAnAssignment(Rule),

    /// The assignment expression is missing its left-hand side.
    #[error("Missing left-hand side in assignment expression")]
    MissingLeftHandSide,

    /// The left-hand side has an invalid rule.
    #[error("Expected assignment_left_hand_side, but found rule: {0:?}")]
    InvalidLeftHandSideRule(Rule),

    /// The left-hand side does not contain an identifier.
    #[error("Left-hand side in assignment expression is empty")]
    EmptyLeftHandSide,

    /// The assignment expression is missing its right-hand side.
    #[error("Missing right-hand side in assignment expression")]
    MissingRightHandSide,

    /// The right-hand side has an invalid rule.
    #[error("Expected assignment_right_hand_side, but found rule: {0:?}")]
    InvalidRightHandSideRule(Rule),

    /// The right-hand side does not contain an expression.
    #[error("Right-hand side in assignment expression is empty")]
    EmptyRightHandSide,

    /// Building the left-hand identifier failed.
    #[error("Failed to build assignment identifier: {0}")]
    BuildIdentifierExpressionError(#[from] BuildIdentifierExpressionError),

    /// Building the right-hand expression failed.
    #[error("Failed to build assignment value expression: {0}")]
    BuildRightHandSideExpressionError(#[from] BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::{Expression, Identifier};

    #[test]
    fn should_build_assignment_expression_when_given_identifier_and_identifier()
    {
        // Arrange
        let input = r#"x = y"#;

        let assignment_rule =
            parse_string_to_rule(input, Rule::assignment_expression)
                .expect("Expected input to be parsed to rules successfully.")
                .next()
                .expect("Expected input to contain an assignment expression.");

        // Act
        let assignment_expression =
            build_assignment_expression(assignment_rule);

        // Assert
        assert!(assignment_expression.is_ok());

        let assignment = assignment_expression.unwrap();
        assert_eq!(assignment.left, Identifier::synthetic("x".to_string()));
        assert!(matches!(
            assignment.right.as_ref(),
            Expression::Identifier(_)
        ));
    }

    #[test]
    fn should_return_error_when_rule_is_not_assignment_expression() {
        // Arrange
        let input = r#"x"#;

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier expression.");

        // Act
        let result = build_assignment_expression(identifier_rule);

        // Assert
        assert!(matches!(
            result,
            Err(BuildAssignmentExpressionError::RuleIsNotAnAssignment(_))
        ));
    }
}
