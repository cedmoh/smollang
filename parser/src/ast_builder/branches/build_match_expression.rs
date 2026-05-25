use crate::{
    ast_builder::{
        BuildAstExpressionError, BuildLiteralExpressionError,
        build_ast_expression, build_identifier_expression,
        build_literal_expression, match_rule_to_expression_builder,
    },
    rule_parser::Rule,
};
use ast::{
    ArrayPattern, ArrayPatternElement, IdentifierPattern, Literal,
    LiteralPattern, Match, MatchArm, Pattern,
};
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed match expression into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// match_expression
///     - match_term > identifier: "operator"
///     - match_arm
///       - pattern > variable_pattern > pattern_term > literal > string_literal > string_text: "+"
///       - expression > operation
///         - operand > identifier: "leftHandSide"
///         - addition: "+"
///         - operand > identifier: "rightHandSide"
///     - match_arm
///       - pattern > variable_pattern > pattern_term > literal > string_literal > string_text: "-"
///       - expression > block > expression > operation
///         - operand > identifier: "leftHandSide"
///         - subtraction: "-"
///         - operand > identifier: "rightHandSide"
/// ```
pub fn build_match_expression(
    pair: Pair<Rule>,
) -> Result<Match, BuildMatchExpressionError> {
    use BuildMatchExpressionError::*;
    use Rule::{match_arm, match_expression, match_term};

    let rule = pair.as_rule();

    if rule != match_expression {
        return Err(RuleIsNotAMatch(rule));
    };

    let mut inner = pair.into_inner();

    // First inner pair must be the match_term.
    let match_term_pair = inner.next().ok_or(MissingMatchTerm)?;

    if match_term_pair.as_rule() != match_term {
        return Err(InvalidMatchTermRule(match_term_pair.as_rule()));
    }

    // The match_term rule wraps an expression-like rule. Unwrap it and build
    // it using the same dispatch as other expression builders.
    let inner_match_term =
        match_term_pair.into_inner().next().ok_or(EmptyMatchTerm)?;

    let expression = match_rule_to_expression_builder(inner_match_term)?;

    let mut match_builder = Match::builder(expression);

    let mut has_branch = false;

    for arm_pair in inner {
        if arm_pair.as_rule() != match_arm {
            return Err(InvalidMatchArmRule(arm_pair.as_rule()));
        }

        let arm = build_match_arm(arm_pair)?;
        match_builder = match_builder.with_branch(arm);
        has_branch = true;
    }

    if !has_branch {
        return Err(NoMatchArms);
    }

    Ok(match_builder.build())
}

/// Converts a `match_arm` pair into an `ast::MatchArm`.
fn build_match_arm(
    pair: Pair<Rule>,
) -> Result<MatchArm, BuildMatchExpressionError> {
    use BuildMatchExpressionError::*;
    use Rule::pattern;

    let mut inner = pair.into_inner();

    let pattern_pair = inner.next().ok_or(MissingPattern)?;

    if pattern_pair.as_rule() != pattern {
        return Err(InvalidPatternRule(pattern_pair.as_rule()));
    }

    let parsed_pattern = build_pattern(pattern_pair)?;

    let body_pair = inner.next().ok_or(MissingArmBody)?;

    let body = build_ast_expression(body_pair)?;

    Ok(MatchArm::new(parsed_pattern, body))
}

/// Converts a `pattern` pair into an `ast::Pattern`.
fn build_pattern(
    pair: Pair<Rule>,
) -> Result<Pattern, BuildMatchExpressionError> {
    use BuildMatchExpressionError::*;
    use Rule::{
        array_pattern, destructuring_pattern, tuple_pattern, variable_pattern,
    };

    let inner = pair.into_inner().next().ok_or(EmptyPattern)?;

    match inner.as_rule() {
        variable_pattern => build_variable_pattern(inner),
        array_pattern => build_array_pattern(inner),
        destructuring_pattern => {
            Err(UnsupportedPatternKind(destructuring_pattern))
        }
        tuple_pattern => Err(UnsupportedPatternKind(tuple_pattern)),
        other => Err(UnexpectedPatternKind(other)),
    }
}

/// Converts a `variable_pattern` pair into an `ast::Pattern`.
fn build_variable_pattern(
    pair: Pair<Rule>,
) -> Result<Pattern, BuildMatchExpressionError> {
    use BuildMatchExpressionError::*;
    use Rule::pattern_term;

    let term_pair = pair.into_inner().next().ok_or(EmptyPattern)?;

    if term_pair.as_rule() != pattern_term {
        return Err(InvalidPatternTermRule(term_pair.as_rule()));
    }

    build_pattern_term(term_pair)
}

/// Converts a `pattern_term` pair into an `ast::Pattern`.
fn build_pattern_term(
    pair: Pair<Rule>,
) -> Result<Pattern, BuildMatchExpressionError> {
    use BuildMatchExpressionError::*;
    use Rule::{identifier, literal, rest_term, wildcard};

    let inner = pair.into_inner().next().ok_or(EmptyPattern)?;

    match inner.as_rule() {
        identifier => {
            let id = build_identifier_expression(inner)
                .map_err(|e| InvalidIdentifierInPattern(e.to_string()))?;

            Ok(Pattern::Identifier(IdentifierPattern::new(id)))
        }
        literal => {
            let parsed_literal = build_literal_expression(inner)?;
            let literal_pattern = literal_to_literal_pattern(parsed_literal);

            Ok(Pattern::Literal(literal_pattern))
        }
        wildcard => Ok(Pattern::Wildcard),
        rest_term => Err(UnsupportedPatternKind(rest_term)),
        other => Err(UnexpectedPatternKind(other)),
    }
}

/// Converts an `array_pattern` pair into an `ast::Pattern::Array`.
fn build_array_pattern(
    pair: Pair<Rule>,
) -> Result<Pattern, BuildMatchExpressionError> {
    use BuildMatchExpressionError::*;
    use Rule::{pattern_term, rest_term};

    let mut builder = ArrayPattern::builder();

    for element in pair.into_inner() {
        match element.as_rule() {
            pattern_term => {
                // pattern_term may wrap a rest_term (`..`) when the grammar
                // chose the pattern_term branch over the direct rest_term
                // branch. Detect that case and emit a Rest element instead.
                let inner =
                    element.clone().into_inner().next().ok_or(EmptyPattern)?;

                if inner.as_rule() == rest_term {
                    builder = builder.with_rest();
                } else {
                    let pattern = build_pattern_term(element)?;
                    builder = builder.with_pattern(pattern);
                }
            }
            rest_term => {
                builder = builder.with_rest();
            }
            other => return Err(UnexpectedPatternKind(other)),
        }
    }

    let array_pattern = builder.build();
    let items: Vec<ArrayPatternElement> = array_pattern.items;

    Ok(Pattern::Array(items))
}

/// Converts an `ast::Literal` to an `ast::LiteralPattern`.
fn literal_to_literal_pattern(literal: Literal) -> LiteralPattern {
    match literal {
        Literal::Nil => LiteralPattern::Nil,
        Literal::Boolean(b) => LiteralPattern::Boolean(b),
        Literal::String(s) => LiteralPattern::String(s),
        Literal::Template(t) => LiteralPattern::Template(t),
        Literal::Integer(i) => LiteralPattern::Integer(i),
        Literal::Decimal(d) => LiteralPattern::Decimal(d),
        Literal::Hexadecimal(h) => LiteralPattern::Hexadecimal(h),
        Literal::Binary(b) => LiteralPattern::Binary(b),
        Literal::Octal(o) => LiteralPattern::Octal(o),
        Literal::Array(_) => {
            todo!("Array literal patterns are not supported yet")
        }
        Literal::Object(_) => {
            todo!("Object literal patterns are not supported yet")
        }
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildMatchExpressionError {
    /// The first rule is not a match expression.
    #[error("Expected a match expression, but found rule: {0:?}")]
    RuleIsNotAMatch(Rule),

    /// The match expression is missing the match term.
    #[error("Missing match term in match expression")]
    MissingMatchTerm,

    /// The match term rule is not a `match_term`.
    #[error("Expected a match_term rule, but found rule: {0:?}")]
    InvalidMatchTermRule(Rule),

    /// The match term has no inner expression.
    #[error("The match term is empty")]
    EmptyMatchTerm,

    /// The match expression has no arms.
    #[error("The match expression has no arms")]
    NoMatchArms,

    /// A match arm rule has an unexpected rule type.
    #[error("Expected a match_arm rule, but found rule: {0:?}")]
    InvalidMatchArmRule(Rule),

    /// A match arm is missing its pattern.
    #[error("Missing pattern in match arm")]
    MissingPattern,

    /// A match arm is missing its body expression.
    #[error("Missing body in match arm")]
    MissingArmBody,

    /// The pattern rule is not a `pattern`.
    #[error("Expected a pattern rule, but found rule: {0:?}")]
    InvalidPatternRule(Rule),

    /// The pattern term rule is not a `pattern_term`.
    #[error("Expected a pattern_term rule, but found rule: {0:?}")]
    InvalidPatternTermRule(Rule),

    /// The pattern has no inner content.
    #[error("The pattern is empty")]
    EmptyPattern,

    /// The pattern kind is unexpected.
    #[error("Unexpected pattern kind: {0:?}")]
    UnexpectedPatternKind(Rule),

    /// The pattern kind is recognized but not yet supported.
    #[error("The pattern kind is not yet supported: {0:?}")]
    UnsupportedPatternKind(Rule),

    /// An identifier within a pattern could not be built.
    #[error("Invalid identifier in pattern: {0}")]
    InvalidIdentifierInPattern(String),

    /// An error occurred while building a literal within a pattern.
    #[error("An error occurred while building a literal in a pattern: {0}")]
    BuildLiteralExpressionError(#[from] BuildLiteralExpressionError),

    /// An error occurred while building an expression within the match.
    #[error("An error occurred while building an expression: {0}")]
    BuildAstExpressionError(#[from] BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::{Expression, Identifier, IntegerLiteral, StringLiteral};

    fn parse_match(input: &str) -> Pair<'_, Rule> {
        parse_string_to_rule(input, Rule::match_expression)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain a match expression.")
    }

    #[test]
    fn should_build_match_expression_with_string_and_identifier_arms() {
        // Arrange
        let input = "operator match\n    'x' -> a\n    other -> b";

        let match_rule = parse_match(input);

        // Act
        let result = build_match_expression(match_rule);

        // Assert
        let expected =
            Match::builder(Identifier::new("operator".to_string()).into())
                .with_branch(MatchArm::new(
                    Pattern::Literal(LiteralPattern::String(
                        StringLiteral::new("x".to_string()),
                    )),
                    Identifier::new("a".to_string()).into(),
                ))
                .with_branch(MatchArm::new(
                    Pattern::Identifier(IdentifierPattern::new(
                        Identifier::new("other".to_string()),
                    )),
                    Identifier::new("b".to_string()).into(),
                ))
                .build();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn should_build_match_expression_with_integer_literal_pattern() {
        // Arrange
        let input = "n match\n    1 -> a";

        let match_rule = parse_match(input);

        // Act
        let result = build_match_expression(match_rule);

        // Assert
        let expected = Match::builder(Identifier::new("n".to_string()).into())
            .with_branch(MatchArm::new(
                Pattern::Literal(LiteralPattern::Integer(IntegerLiteral::new(
                    1,
                ))),
                Identifier::new("a".to_string()).into(),
            ))
            .build();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn should_build_match_expression_with_identifier_binding_pattern() {
        // Arrange
        let input = "value match\n    x -> x";

        let match_rule = parse_match(input);

        // Act
        let result = build_match_expression(match_rule);

        // Assert
        let expected =
            Match::builder(Identifier::new("value".to_string()).into())
                .with_branch(MatchArm::new(
                    Pattern::Identifier(IdentifierPattern::new(
                        Identifier::new("x".to_string()),
                    )),
                    Identifier::new("x".to_string()).into(),
                ))
                .build();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn should_build_match_expression_with_array_pattern() {
        // Arrange
        let input = "value match\n    [x, y, ..] -> x";

        let match_rule = parse_match(input);

        // Act
        let result = build_match_expression(match_rule);

        // Assert
        let array_items = ArrayPattern::builder()
            .with_pattern(Pattern::Identifier(IdentifierPattern::new(
                Identifier::new("x".to_string()),
            )))
            .with_pattern(Pattern::Identifier(IdentifierPattern::new(
                Identifier::new("y".to_string()),
            )))
            .with_rest()
            .build()
            .items;

        let expected =
            Match::builder(Identifier::new("value".to_string()).into())
                .with_branch(MatchArm::new(
                    Pattern::Array(array_items),
                    Identifier::new("x".to_string()).into(),
                ))
                .build();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn should_build_match_expression_with_block_body() {
        // Arrange
        let input = "value match\n    'a' -> (b)";

        let match_rule = parse_match(input);

        // Act
        let result = build_match_expression(match_rule);

        // Assert
        let match_expression =
            result.expect("Expected match to be built successfully.");

        assert_eq!(match_expression.branches.len(), 1);
        assert!(matches!(
            match_expression.branches[0].body,
            Expression::Block(_)
        ));
    }

    #[test]
    fn should_return_error_when_rule_is_not_match_expression() {
        // Arrange
        let input = "myIdentifier";

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier.");

        // Act
        let result = build_match_expression(identifier_rule);

        // Assert
        assert_eq!(
            result,
            Err(BuildMatchExpressionError::RuleIsNotAMatch(Rule::identifier))
        );
    }
}
