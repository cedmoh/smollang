use crate::{
    ast_builder::{BuildAstExpressionError, build_ast_expression},
    into_ast_span::IntoAstSpan,
    rule_parser::Rule,
};
use ast::Return;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed return expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// returned_expression > expression > identifier: "hello"
/// ```
///
/// ```pest
/// returned_expression > "ret"
/// ```
pub fn build_return_expression(
    pair: Pair<Rule>,
) -> Result<Return, BuildReturnExpressionError> {
    use BuildReturnExpressionError::*;
    use Rule::*;

    let rule = pair.as_rule();

    if rule != returned_expression {
        return Err(RuleIsNotAReturn(rule));
    };

    let span = pair.as_span().into_ast_span();

    let mut inner_rules = pair.into_inner();

    let next_inner_rule = inner_rules.next();

    let Some(next_inner_rule) = next_inner_rule else {
        // This means we have a return statement without an expression, e.g.,
        // `ret`
        return Ok(Return::new(None, span));
    };

    match build_ast_expression(next_inner_rule) {
        Ok(expr) => Ok(Return::new(Some(Box::new(expr)), span)),
        Err(error) => Err(BuildExpressionError(error)),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildReturnExpressionError {
    /// The first rule is not a return expression.
    #[error("Expected a return expression, but found rule: {0:?}")]
    RuleIsNotAReturn(Rule),

    /// An error occurred while building the returned expression.
    #[error("An error occurred while building the returned expression: {0}")]
    BuildExpressionError(BuildAstExpressionError),
}
