use crate::{ast_builder::build_block_expression, rule_parser::Rule};
use ast::Expression;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts a sequence of expression rules into an abstract syntax tree (AST) representation of an expression.
pub fn build_ast_expression(pair: Pair<Rule>) -> Result<Expression, BuildAstExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::expression {
        return Err(BuildAstExpressionError::RuleIsNotAnExpression(rule));
    };

    let inner = pair.into_inner();

    let Some(inner_expression) = inner.into_iter().next() else {
        return Err(BuildAstExpressionError::EmptyExpression);
    };

    match inner_expression.as_rule() {
        Rule::block => build_block_expression(inner_expression)
            .map(|block| Expression::Block(block))
            .map_err(|error| BuildAstExpressionError::BuildInnerAstError(error.to_string())),
        Rule::then_expression => todo!(),
        Rule::pipe_expression => todo!(),
        Rule::operation => todo!(),
        Rule::assignment_expression => todo!(),
        Rule::declaration => todo!(),
        Rule::match_expression => todo!(),
        Rule::call => todo!(),
        Rule::member => todo!(),
        Rule::identifier => todo!(),
        Rule::literal => todo!(),
        Rule::expression_in_parenthesis => todo!(),
        Rule::returned_expression => todo!(),
        Rule::broken_expression => todo!(),
        Rule::continue_expression => todo!(),
        x => Err(BuildAstExpressionError::UnrecognizedExpression(x)),
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum BuildAstExpressionError {
    /// The expression is empty.
    #[error("The expression is empty.")]
    EmptyExpression,

    /// The first rule is not an expression.
    #[error("Expected an expression, but found rule: {0:?}")]
    RuleIsNotAnExpression(Rule),

    /// The expression is not recognized.
    #[error("Unrecognized expression. Found rule: {0:?}")]
    UnrecognizedExpression(Rule),

    /// An error occurred while building an inner AST expression.
    #[error("An error occurred while building an inner AST expression: {0}")]
    BuildInnerAstError(String),
}
