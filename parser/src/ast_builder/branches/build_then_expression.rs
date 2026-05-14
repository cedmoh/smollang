use crate::rule_parser::Rule;
use ast::Then;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed then expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// - then_expression
///   - expression > condition
///   - expression > then_body
///   - expression > else_body (optional)
/// ```
pub fn build_then_expression(
    pair: Pair<Rule>,
) -> Result<Then, BuildThenExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::then_expression {
        return Err(BuildThenExpressionError::RuleIsNotAThenExpression(rule));
    };

    // TODO: Implement the actual parsing logic
    Err(BuildThenExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildThenExpressionError {
    /// The first rule is not a then expression.
    #[error("Expected a then expression, but found rule: {0:?}")]
    RuleIsNotAThenExpression(Rule),

    /// This expression cannot be built yet, as it is unimplemented.
    #[error("This expression cannot be built yet, as it is unimplemented.")]
    Unimplemented,
}
