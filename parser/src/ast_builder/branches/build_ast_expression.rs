use crate::{ast_builder::match_rule_to_expression_builder, rule_parser::Rule};
use ast::Expression;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts a sequence of expression rules into an abstract syntax tree (AST)
/// representation of an expression.
///
/// If the provided rule is an `expression`, it extracts the inner expression
/// and matches it to the appropriate expression builder function.
pub fn build_ast_expression(
    pair: Pair<Rule>,
) -> Result<Expression, BuildAstExpressionError> {
    use BuildAstExpressionError::*;
    use Rule::*;

    let rule = pair.as_rule();

    match rule {
        // The rule is an expression, so we need to extract the inner expression
        expression => {
            let inner = pair.into_inner();

            let Some(inner_expression) = inner.into_iter().next() else {
                return Err(EmptyExpression);
            };

            match_rule_to_expression_builder(inner_expression)
        }
        _ => match_rule_to_expression_builder(pair),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildAstExpressionError {
    #[error("The expression is empty.")]
    /// The expression is empty.
    EmptyExpression,

    #[error("Unrecognized expression. Found rule : {0:?}")]
    /// The expression is unrecognized.
    UnrecognizedExpression(Rule),

    #[error("An error occurred while building an expression variant : {0}")]
    /// An error occurred while building an expression variant.
    BuildExpressionVariantError(String),

    #[error(
        "This expression cannot be built yet, as it is unimplemented. Rule: {0:?}"
    )]
    /// This expression cannot be built yet, as it is unimplemented.
    Unimplemented(Rule),
}
