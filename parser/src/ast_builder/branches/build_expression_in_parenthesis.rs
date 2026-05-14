use crate::{
    ast_builder::{build_ast_expression, BuildAstExpressionError},
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
/// - expression_in_parenthesis
///   - expression
/// ```
pub fn build_expression_in_parenthesis(
    pair: Pair<Rule>,
) -> Result<Expression, BuildExpressionInParenthesisError> {
    let rule = pair.as_rule();

    if rule != Rule::expression_in_parenthesis {
        return Err(BuildExpressionInParenthesisError::RuleIsNotAnExpressionInParenthesis(rule));
    };

    let mut inner = pair.into_inner();

    let Some(inner_expression) = inner.next() else {
        return Err(BuildExpressionInParenthesisError::EmptyParentheses);
    };

    build_ast_expression(inner_expression)
        .map_err(BuildExpressionInParenthesisError::BuildAstExpressionError)
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildExpressionInParenthesisError {
    /// The first rule is not an expression in parenthesis.
    #[error("Expected an expression in parenthesis, but found rule: {0:?}")]
    RuleIsNotAnExpressionInParenthesis(Rule),

    /// The parentheses are empty.
    #[error("Expected an expression in parenthesis, but found empty parentheses.")]
    EmptyParentheses,

    /// An error occurred while building the inner AST expression.
    #[error("An error occurred while building the inner AST expression: {0}")]
    BuildAstExpressionError(BuildAstExpressionError),
}
