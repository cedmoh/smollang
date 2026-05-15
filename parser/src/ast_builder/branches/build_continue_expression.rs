use crate::rule_parser::Rule;
use ast::Return;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed continue expression into an AST
/// representation.
///
/// Note: Currently, there is no dedicated Continue type in the AST, so this
/// returns a Return type as a placeholder. This should be updated when a
/// proper Continue type is added to the AST.
///
/// # Examples
///
/// ```pest
/// - continue_expression
/// ```
pub fn build_continue_expression(
    pair: Pair<Rule>,
) -> Result<Return, BuildContinueExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::continue_expression {
        return Err(BuildContinueExpressionError::RuleIsNotAContinue(rule));
    };

    // TODO: Implement the actual parsing logic
    // TODO: Create a dedicated Continue type in the AST
    Err(BuildContinueExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildContinueExpressionError {
    /// The first rule is not a continue expression.
    #[error("Expected a continue expression, but found rule: {0:?}")]
    RuleIsNotAContinue(Rule),

    /// The continue expression cannot be built yet, as it is unimplemented.
    #[error(
        "The continue expression cannot be built yet, as it is unimplemented."
    )]
    Unimplemented,
}
