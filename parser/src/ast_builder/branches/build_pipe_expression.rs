use crate::rule_parser::Rule;
use ast::Pipe;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed pipe expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// - pipe_expression
///   - expression > first
///   - expression > second
///   - expression > third (optional, can chain multiple)
/// ```
pub fn build_pipe_expression(
    pair: Pair<Rule>,
) -> Result<Pipe, BuildPipeExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::pipe_expression {
        return Err(BuildPipeExpressionError::RuleIsNotAPipeExpression(rule));
    };

    // TODO: Implement the actual parsing logic
    Err(BuildPipeExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildPipeExpressionError {
    /// The first rule is not a pipe expression.
    #[error("Expected a pipe expression, but found rule: {0:?}")]
    RuleIsNotAPipeExpression(Rule),

    /// This expression cannot be built yet, as it is unimplemented.
    #[error("This expression cannot be built yet, as it is unimplemented.")]
    Unimplemented,
}
