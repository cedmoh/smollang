use crate::{ast_builder::match_rule_to_expression_builder, rule_parser::Rule};
use ast::Expression;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts a sequence of expression rules into an abstract syntax tree (AST)
/// representation of an expression.
pub fn build_ast_expression(
    pair: Pair<Rule>,
) -> Result<Expression, BuildAstExpressionError> {
    use BuildAstExpressionError::*;
    use Rule::*;

    let rule = pair.as_rule();

    if rule != expression {
        return Err(RuleIsNotAnExpression(rule));
    };

    let inner = pair.into_inner();

    let Some(inner_expression) = inner.into_iter().next() else {
        return Err(EmptyExpression);
    };

    match_rule_to_expression_builder(inner_expression)
}

#[derive(Debug, PartialEq, Error)]
pub enum BuildAstExpressionError {
    #[error("The expression is empty.")]
    EmptyExpression,

    #[error("Expected an expression, but found rule: {0:?}")]
    RuleIsNotAnExpression(Rule),

    #[error("Unrecognized expression. Found rule: {0:?}")]
    UnrecognizedExpression(Rule),

    #[error("An error occurred while building an inner AST expression: {0}")]
    BuildInnerAstError(String),

    #[error("This expression cannot be built yet, as it is unimplemented.")]
    Unimplemented(Rule),
}
