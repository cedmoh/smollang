use crate::rule_parser::Rule;
use ast::Match;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed match expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// - match_expression
///   - expression > match_term
///   - match_arms
///     - match_arm
///       - pattern
///       - block > body
/// ```
pub fn build_match_expression(
    pair: Pair<Rule>,
) -> Result<Match, BuildMatchExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::match_expression {
        return Err(BuildMatchExpressionError::RuleIsNotAMatch(rule));
    };

    // TODO: Implement the actual parsing logic
    Err(BuildMatchExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildMatchExpressionError {
    /// The first rule is not a match expression.
    #[error("Expected a match expression, but found rule: {0:?}")]
    RuleIsNotAMatch(Rule),

    /// This expression cannot be built yet, as it is unimplemented.
    #[error("This expression cannot be built yet, as it is unimplemented.")]
    Unimplemented,
}
