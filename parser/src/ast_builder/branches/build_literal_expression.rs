use crate::rule_parser::Rule;
use ast::Literal;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed literal expression into an AST
/// representation.
///
/// # Example
///
/// ```
/// literal > nil_literal > nil: "nil"
/// ```
/// ```
/// literal > boolean_literal > true: "true"
/// ```
/// ```
/// literal > string_literal > string_text: "hello"
/// ```
/// ```
/// literal > decimal_literal: "15"
/// ```
/// ```
/// literal > array_literal > array_entries > array_entry > expression > ...
/// ```
/// ```
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
        nil_literal => Err(Unimplemented(inner_literal.as_rule())),
        boolean_literal => Err(Unimplemented(inner_literal.as_rule())),
        string_literal => Err(Unimplemented(inner_literal.as_rule())),
        decimal_literal => Err(Unimplemented(inner_literal.as_rule())),
        array_literal => Err(Unimplemented(inner_literal.as_rule())),
        object_literal => Err(Unimplemented(inner_literal.as_rule())),
        octal_literal => Err(Unimplemented(inner_literal.as_rule())),
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

    /// An unexpected inner literal was found in the literal expression.
    #[error("Unexpected inner literal found in the literal expression: {0:?}")]
    UnexpectedInnerLiteral(Rule),

    /// This literal expression cannot be built yet, as it is unimplemented.
    #[error(
        "This literal expression cannot be built yet, as it is unimplemented. Literal: {0:?}"
    )]
    Unimplemented(Rule),
}
