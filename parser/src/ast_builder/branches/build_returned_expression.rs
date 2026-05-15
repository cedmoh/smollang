use crate::rule_parser::Rule;
use ast::Return;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed return expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// - returned_expression
///   - expression (optional)
/// ```
pub fn build_returned_expression(
    pair: Pair<Rule>,
) -> Result<Return, BuildReturnedExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::returned_expression {
        return Err(BuildReturnedExpressionError::RuleIsNotAReturn(rule));
    };

    // TODO: Implement the actual parsing logic
    Err(BuildReturnedExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildReturnedExpressionError {
    /// The first rule is not a return expression.
    #[error("Expected a return expression, but found rule: {0:?}")]
    RuleIsNotAReturn(Rule),

    /// The return expression cannot be built yet, as it is unimplemented.
    #[error(
        "The return expression cannot be built yet, as it is unimplemented."
    )]
    Unimplemented,
}
