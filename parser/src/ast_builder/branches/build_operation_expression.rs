use crate::rule_parser::Rule;
use ast::Dyadic;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed operation expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// - operation
///   - operand > left
///   - operator
///   - operand > right
/// ```
pub fn build_operation_expression(
    pair: Pair<Rule>,
) -> Result<Dyadic, BuildOperationExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::operation {
        return Err(BuildOperationExpressionError::RuleIsNotAnOperation(rule));
    };

    // TODO: Implement the actual parsing logic using Pratt parser
    Err(BuildOperationExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildOperationExpressionError {
    /// The first rule is not an operation.
    #[error("Expected an operation expression, but found rule: {0:?}")]
    RuleIsNotAnOperation(Rule),

    /// This expression cannot be built yet, as it is unimplemented.
    #[error("This expression cannot be built yet, as it is unimplemented.")]
    Unimplemented,
}
