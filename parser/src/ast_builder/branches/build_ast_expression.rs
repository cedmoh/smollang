use crate::{
    ast_builder::{
        build_block_expression, build_call_expression,
        build_identifier_expression,
    },
    rule_parser::Rule,
};
use ast::Expression;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts a sequence of expression rules into an abstract syntax tree (AST)
/// representation of an expression.
pub fn build_ast_expression(
    pair: Pair<Rule>,
) -> Result<Expression, BuildAstExpressionError> {
    use BuildAstExpressionError::*;
    use Expression::*;
    use Rule::*;

    let rule = pair.as_rule();

    if rule != expression {
        return Err(RuleIsNotAnExpression(rule));
    };

    let inner = pair.into_inner();

    let Some(inner_expression) = inner.into_iter().next() else {
        return Err(EmptyExpression);
    };

    match inner_expression.as_rule() {
        block => build_block_expression(inner_expression)
            .map(|b| Block(b))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        then_expression => todo!(),

        pipe_expression => todo!(),

        operation => todo!(),

        assignment_expression => todo!(),

        declaration => todo!(),

        match_expression => todo!(),

        call => build_call_expression(inner_expression)
            .map(|c| FunctionCall(c))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        member => todo!(),

        identifier => build_identifier_expression(inner_expression)
            .map(|i| Identifier(i))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        literal => todo!(),

        expression_in_parenthesis => todo!(),

        returned_expression => todo!(),

        broken_expression => todo!(),

        continue_expression => todo!(),

        x => Err(UnrecognizedExpression(x)),
    }
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
