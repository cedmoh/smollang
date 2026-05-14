use crate::{
    ast_builder::{BuildAstExpressionError, build_ast_expression},
    rule_parser::Rule,
};
use ast::Expression;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed expression in parenthesis into an AST
/// representation. This simply unwraps the parentheses and returns the inner
/// expression.
///
/// # Example
///
/// ```
/// - dynamic_key_expression
///   - expression
/// ```
pub fn build_dynamic_key_expression(
    pair: Pair<Rule>,
) -> Result<Expression, BuildExpressionInParenthesisError> {
    use BuildExpressionInParenthesisError::*;
    use Rule::dynamic_key_expression;

    let rule = pair.as_rule();

    if rule != dynamic_key_expression {
        return Err(RuleIsNotAnExpressionInParenthesis(rule));
    };

    let mut inner = pair.into_inner();

    let Some(inner_expression) = inner.next() else {
        return Err(EmptyParentheses);
    };

    build_ast_expression(inner_expression).map_err(BuildAstExpressionError)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildExpressionInParenthesisError {
    /// The first rule is not an expression in parenthesis.
    #[error("Expected an expression in parenthesis, but found rule: {0:?}")]
    RuleIsNotAnExpressionInParenthesis(Rule),

    /// The parentheses are empty.
    #[error(
        "Expected an expression in parenthesis, but found empty parentheses."
    )]
    EmptyParentheses,

    /// An error occurred while building the inner AST expression.
    #[error("An error occurred while building the inner AST expression: {0}")]
    BuildAstExpressionError(BuildAstExpressionError),
}
