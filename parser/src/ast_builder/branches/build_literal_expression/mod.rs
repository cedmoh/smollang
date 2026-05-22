mod build_boolean_literal;
mod build_decimal_literal;
mod build_nil_literal;
mod build_octal_literal;
mod build_string_literal;

pub use build_boolean_literal::build_boolean_literal;
pub use build_decimal_literal::build_decimal_literal;
pub use build_nil_literal::build_nil_literal;
pub use build_octal_literal::build_octal_literal;
pub use build_string_literal::build_string_literal;

use crate::rule_parser::Rule;
use ast::Literal;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed literal expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// literal > nil_literal > nil: "nil"
/// ```
/// ```pest
/// literal > boolean_literal > true: "true"
/// ```
/// ```pest
/// literal > string_literal > string_text: "hello"
/// ```
/// ```pest
/// literal > decimal_literal: "15"
/// ```
/// ```pest
/// literal > array_literal > array_entries > array_entry > expression > ...
/// ```
/// ```pest
/// literal > object_literal > object_entries > object_entry
///   - object_key > identifier: "hello"
///   - object_value > expression > ...
/// ```
pub fn build_literal_expression(
    pair: Pair<Rule>,
) -> Result<Literal, BuildLiteralExpressionError> {
    use BuildLiteralExpressionError::*;
    use Rule::*;

    let rule = pair.as_rule();

    if rule != literal {
        return Err(RuleIsNotALiteral(rule));
    };

    let Some(inner_literal) = pair.into_inner().next() else {
        return Err(NoInnerLiteral);
    };

    match inner_literal.as_rule() {
        nil_literal => build_nil_literal(inner_literal)
            .map_err(|e| BuildLiteralVariantError(e.to_string())),
        boolean_literal => build_boolean_literal(inner_literal)
            .map_err(|e| BuildLiteralVariantError(e.to_string()))
            .map(Literal::Boolean),
        string_literal => build_string_literal(inner_literal)
            .map_err(|e| BuildLiteralVariantError(e.to_string()))
            .map(|s| s.into()),
        decimal_literal => build_decimal_literal(inner_literal)
            .map_err(|e| BuildLiteralVariantError(e.to_string())),
        array_literal => Err(Unimplemented(inner_literal.as_rule())),
        object_literal => Err(Unimplemented(inner_literal.as_rule())),
        octal_literal => build_octal_literal(inner_literal)
            .map_err(|e| BuildLiteralVariantError(e.to_string()))
            .map(|o| o.into()),
        //
        _ => Err(UnexpectedInnerLiteral(inner_literal.as_rule())),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildLiteralExpressionError {
    /// The first rule is not a literal expression.
    #[error("Expected a literal expression, but found rule: {0:?}")]
    RuleIsNotALiteral(Rule),

    /// No inner literal was found in the literal expression.
    #[error("No inner literal found in the literal expression.")]
    NoInnerLiteral,

    #[error("Failed to build a literal variant: {0}")]
    BuildLiteralVariantError(String),

    /// An unexpected inner literal was found in the literal expression.
    #[error("Unexpected inner literal found in the literal expression: {0:?}")]
    UnexpectedInnerLiteral(Rule),

    /// This literal expression cannot be built yet, as it is unimplemented.
    #[error(
        "This literal expression cannot be built yet, as it is unimplemented. Literal: {0:?}"
    )]
    Unimplemented(Rule),
}
