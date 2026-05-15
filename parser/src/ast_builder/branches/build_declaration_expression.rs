use crate::rule_parser::Rule;
use ast::VariableDeclaration;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed declaration expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// - declaration
///   - variable_declaration (mutable or immutable)
///     - identifier
///     - expression > initial_value (optional)
/// ```
pub fn build_declaration_expression(
    pair: Pair<Rule>,
) -> Result<VariableDeclaration, BuildDeclarationExpressionError> {
    let rule = pair.as_rule();

    // Note: declaration rule can be either mutable_variable_declaration or
    // immutable_variable_declaration
    match rule {
        Rule::mutable_variable_declaration
        | Rule::immutable_variable_declaration => {
            // TODO: Implement the actual parsing logic
            Err(BuildDeclarationExpressionError::Unimplemented)
        }
        _ => Err(BuildDeclarationExpressionError::RuleIsNotADeclaration(rule)),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildDeclarationExpressionError {
    /// The first rule is not a declaration.
    #[error("Expected a declaration expression, but found rule: {0:?}")]
    RuleIsNotADeclaration(Rule),

    /// The declaration expression cannot be built yet, as it is unimplemented.
    #[error(
        "The declaration expression cannot be built yet, as it is unimplemented."
    )]
    Unimplemented,
}
