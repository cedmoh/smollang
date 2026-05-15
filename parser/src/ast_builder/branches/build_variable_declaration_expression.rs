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
pub fn build_variable_declaration_expression(
    pair: Pair<Rule>,
) -> Result<VariableDeclaration, BuildVariableDeclarationExpressionError> {
    use BuildVariableDeclarationExpressionError::*;
    use Rule::{immutable_variable_declaration, mutable_variable_declaration};

    let rule = pair.as_rule();

    // Note: declaration rule can be either mutable_variable_declaration or
    // immutable_variable_declaration
    match rule {
        mutable_variable_declaration | immutable_variable_declaration => {
            // TODO: Implement the actual parsing logic
            Err(Unimplemented)
        }
        _ => Err(RuleIsNotAVariableDeclaration(rule)),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildVariableDeclarationExpressionError {
    /// The first rule is not a variable declaration.
    #[error(
        "Expected a variable declaration expression, but found rule: {0:?}"
    )]
    RuleIsNotAVariableDeclaration(Rule),

    /// The variable declaration expression cannot be built yet, as it is
    /// unimplemented.
    #[error(
        "The variable declaration expression cannot be built yet, as it is unimplemented."
    )]
    Unimplemented,
}
