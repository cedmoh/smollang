use ast::Expression;
use pest::iterators::Pair;

use crate::{
    ast_builder::{
        BuildAstExpressionError, build_assignment_expression,
        build_ast_expression, build_block_expression, build_broken_expression,
        build_call_expression, build_continue_expression,
        build_dynamic_key_expression, build_function_declaration_expression,
        build_identifier_expression, build_literal_expression,
        build_match_expression, build_member_expression,
        build_operation_expression, build_pipe_expression,
        build_returned_expression, build_then_expression,
        build_variable_declaration_expression,
    },
    rule_parser::Rule,
};

/// Matches a pest rule to the appropriate AST expression builder function,
/// and converts the inner expression into an AST expression.
/// This is used to convert the inner expressions of various rules
/// (e.g., block, then_expression, pipe_expression, etc.) into AST expressions.
pub fn match_rule_to_expression_builder(
    inner_expression: Pair<'_, Rule>,
) -> Result<Expression, BuildAstExpressionError> {
    use BuildAstExpressionError::*;
    use Expression::*;
    use Rule::*;

    match inner_expression.as_rule() {
        expression => build_ast_expression(inner_expression)
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        block => build_block_expression(inner_expression)
            .map(|b| Block(b))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        then_expression => build_then_expression(inner_expression)
            .map(|t| Then(t))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        pipe_expression => build_pipe_expression(inner_expression)
            .map(|p| Pipe(p))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        operation => build_operation_expression(inner_expression)
            .map(|d| Dyadic(d))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        assignment_expression => build_assignment_expression(inner_expression)
            .map(|a| Assignment(a))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        variable_declaration => build_variable_declaration_expression(inner_expression)
            .map(|v| VariableDeclaration(v))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        function_declaration => build_function_declaration_expression(inner_expression)
            .map(|v| FunctionDeclaration(v))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        match_expression => build_match_expression(inner_expression)
            .map(|m| Match(m))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        call => build_call_expression(inner_expression)
            .map(|c| FunctionCall(c))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        member => build_member_expression(inner_expression)
            .map(|m| Member(m))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        identifier => build_identifier_expression(inner_expression)
            .map(|i| Identifier(i))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        literal => build_literal_expression(inner_expression)
            .map(|l| Literal(l))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        dynamic_key_expression => build_dynamic_key_expression(inner_expression)
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        returned_expression => build_returned_expression(inner_expression)
            .map(|r| Return(r))
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        broken_expression => build_broken_expression(inner_expression)
            .map(|_| {
                todo!("Break expression is not yet supported in Expression enum")
            })
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        continue_expression => build_continue_expression(inner_expression)
            .map(|_| {
                todo!("Continue expression is not yet supported in Expression enum")
            })
            .map_err(|error| BuildExpressionVariantError(error.to_string())),
        // ---
        x => Err(UnrecognizedExpression(x)),
    }
}
