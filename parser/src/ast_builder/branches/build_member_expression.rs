use crate::rule_parser::Rule;
use ast::Member;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed member expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// - member
///   - identifier/call/literal > first
///   - identifier/call/literal > second
///   - identifier/call/literal > third (optional, can chain multiple)
/// ```
pub fn build_member_expression(
    pair: Pair<Rule>,
) -> Result<Member, BuildMemberExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::member {
        return Err(BuildMemberExpressionError::RuleIsNotAMember(rule));
    };

    // TODO: Implement the actual parsing logic
    Err(BuildMemberExpressionError::Unimplemented)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildMemberExpressionError {
    /// The first rule is not a member expression.
    #[error("Expected a member expression, but found rule: {0:?}")]
    RuleIsNotAMember(Rule),

    /// This expression cannot be built yet, as it is unimplemented.
    #[error("This expression cannot be built yet, as it is unimplemented.")]
    Unimplemented,
}
