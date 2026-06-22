use crate::{
    ast_builder::{BuildAstExpressionError, build_ast_expression},
    into_ast_span::IntoAstSpan,
    rule_parser::Rule,
};
use ast::Block;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed block expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// - block
///   - expression > ...
///   - expression > ...
/// ```
pub fn build_block_expression(
    pair: Pair<Rule>,
) -> Result<Block, BuildBlockExpressionError> {
    let rule = pair.as_rule();

    if rule != Rule::block {
        return Err(BuildBlockExpressionError::RuleIsNotABlock(rule));
    };

    let span = pair.as_span().into_ast_span();

    let inner = pair.into_inner();

    let mut block_builder = Block::builder();

    for inner_expression in inner {
        match build_ast_expression(inner_expression) {
            Ok(expression) => block_builder.add_expression(expression),
            Err(e) => {
                return Err(
                    BuildBlockExpressionError::BuildAstExpressionError(e),
                );
            }
        };
    }

    Ok(block_builder.with_span(span).build())
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildBlockExpressionError {
    /// The first rule is not a block.
    #[error("Expected a block expression, but found rule: {0:?}")]
    RuleIsNotABlock(Rule),

    /// An error occurred while building an expression within the block.
    #[error(
        "An error occurred while building an expression within the block: {0}"
    )]
    BuildAstExpressionError(#[from] BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::Span;

    #[test]
    fn should_build_empty_block() {
        // Arrange
        let input = r#"()"#;

        let block_rule = parse_string_to_rule(input, Rule::block)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a block.");

        // Act
        let block_expression = build_block_expression(block_rule);

        // Assert
        assert!(block_expression.is_ok());
        let block_expression = block_expression.unwrap();
        assert!(block_expression.body.items.is_empty());
        assert_ne!(block_expression.span, Span::DUMMY);
    }
}
