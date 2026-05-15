use crate::rule_parser::Rule;
use ast::Assignment;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed assignment expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// - assignment_expression
///   - identifier
///   - expression > value
/// ```
pub fn build_assignment_expression(
    pair: Pair<Rule>,
) -> Result<Assignment, BuildAssignmentExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::assignment_expression {
        return Err(BuildAssignmentExpressionError::RuleIsNotAnAssignment(
            rule,
        ));
    };

    // TODO: Implement the actual parsing logic
    Err(BuildAssignmentExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildAssignmentExpressionError {
    /// The first rule is not an assignment expression.
    #[error("Expected an assignment expression, but found rule: {0:?}")]
    RuleIsNotAnAssignment(Rule),

    /// The assignment expression cannot be built yet, as it is unimplemented.
    #[error(
        "Assignment expression cannot be built yet, as it is unimplemented."
    )]
    Unimplemented,
}
