use crate::{
    ast_builder::{
        build_assignment_expression, build_block_expression,
        build_broken_expression, build_call_expression,
        build_continue_expression, build_declaration_expression,
        build_expression_in_parenthesis, build_identifier_expression,
        build_literal_expression, build_match_expression,
        build_member_expression, build_operation_expression,
        build_pipe_expression, build_returned_expression,
        build_then_expression,
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

        then_expression => build_then_expression(inner_expression)
            .map(|_| {
                // TODO: Add Then variant to Expression enum
                todo!("Then expression is not yet supported in Expression enum")
            })
            .map_err(|error| BuildInnerAstError(error.to_string())),

        pipe_expression => build_pipe_expression(inner_expression)
            .map(|_| {
                // TODO: Add Pipe variant to Expression enum
                todo!("Pipe expression is not yet supported in Expression enum")
            })
            .map_err(|error| BuildInnerAstError(error.to_string())),

        operation => build_operation_expression(inner_expression)
            .map(|d| Dyadic(d))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        assignment_expression => build_assignment_expression(inner_expression)
            .map(|a| Assignment(a))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        declaration => build_declaration_expression(inner_expression)
            .map(|v| VariableDeclaration(v))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        match_expression => build_match_expression(inner_expression)
            .map(|m| Match(m))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        call => build_call_expression(inner_expression)
            .map(|c| FunctionCall(c))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        member => build_member_expression(inner_expression)
            .map(|m| Member(m))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        identifier => build_identifier_expression(inner_expression)
            .map(|i| Identifier(i))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        literal => build_literal_expression(inner_expression)
            .map(|l| Literal(l))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        expression_in_parenthesis => build_expression_in_parenthesis(inner_expression)
            .map_err(|error| BuildInnerAstError(error.to_string())),

        returned_expression => build_returned_expression(inner_expression)
            .map(|r| Return(r))
            .map_err(|error| BuildInnerAstError(error.to_string())),

        broken_expression => build_broken_expression(inner_expression)
            .map(|_| {
                // TODO: Add Break variant to Expression enum
                todo!("Break expression is not yet supported in Expression enum")
            })
            .map_err(|error| BuildInnerAstError(error.to_string())),

        continue_expression => build_continue_expression(inner_expression)
            .map(|_| {
                // TODO: Add Continue variant to Expression enum
                todo!("Continue expression is not yet supported in Expression enum")
            })
            .map_err(|error| BuildInnerAstError(error.to_string())),

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
