use crate::rule_parser::Rule;
use ast::Return;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed break expression into an AST
/// representation.
///
/// Note: Currently, there is no dedicated Break type in the AST, so this
/// returns a Return type as a placeholder. This should be updated when a
/// proper Break type is added to the AST.
///
/// # Examples
///
/// ```pest
/// - broken_expression
///   - expression (optional)
/// ```
pub fn build_broken_expression(
    pair: Pair<Rule>,
) -> Result<Return, BuildBrokenExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::broken_expression {
        return Err(BuildBrokenExpressionError::RuleIsNotABreak(rule));
    };

    // TODO: Implement the actual parsing logic
    // TODO: Create a dedicated Break type in the AST
    Err(BuildBrokenExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildBrokenExpressionError {
    /// The first rule is not a break expression.
    #[error("Expected a break expression, but found rule: {0:?}")]
    RuleIsNotABreak(Rule),

    /// The break expression cannot be built yet, as it is unimplemented.
    #[error(
        "The break expression cannot be built yet, as it is unimplemented."
    )]
    Unimplemented,
}
