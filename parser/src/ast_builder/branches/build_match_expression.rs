use crate::rule_parser::Rule;
use ast::Match;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed match expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// match_expression
///     - match_term > identifier: "operator"
///     - match_arm
///       - pattern > variable_pattern > pattern_term > literal > string_literal > string_text: "+"
///       - expression > operation
///         - operand > identifier: "leftHandSide"
///         - addition: "+"
///         - operand > identifier: "rightHandSide"
///     - match_arm
///       - pattern > variable_pattern > pattern_term > literal > string_literal > string_text: "-"
///       - expression > block > expression > operation
///         - operand > identifier: "leftHandSide"
///         - subtraction: "-"
///         - operand > identifier: "rightHandSide"
/// ```
pub fn build_match_expression(
    pair: Pair<Rule>,
) -> Result<Match, BuildMatchExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::match_expression {
        return Err(BuildMatchExpressionError::RuleIsNotAMatch(rule));
    };

    todo!("Implement build_match_expression");
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildMatchExpressionError {
    /// The first rule is not a match expression.
    #[error("Expected a match expression, but found rule: {0:?}")]
    RuleIsNotAMatch(Rule),
}
