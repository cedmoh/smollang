use crate::{
    ast_builder::build_ast_expression,
    rule_parser::{MEMBER_PRATT_PARSER, Rule},
};
use ast::{Expression, FunctionCall, Member};
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed member expression into an AST
/// representation.
///
/// # Examples
///
/// `yes().this.is.amazing(6,7).(hello)('this is an argument')` would be
/// parsed into the following rule tree:
///
/// ```pest
/// member
///   - member_segment > identifier: "yes"
///   - member_infix_operator > member_call > call_arguments: "()"
///   - member_segment > identifier: "this"
///   - member_infix_operator > member_dot: "."
///   - member_segment > identifier: "is"
///   - member_infix_operator > member_dot: "."
///   - member_segment > identifier: "amazing"
///   - member_infix_operator > member_call > call_arguments
///     - expression > literal > integer_literal: "6"
///     - expression > literal > integer_literal: "7"
///   - member_segment > block > expression > identifier: "hello"
///   - member_postfix_operator > member_call_postfix > call_arguments > expression > literal > string_literal > string_text: "this is an argument"
/// ```
pub fn build_member_expression(
    pair: Pair<Rule>,
) -> Result<Member, BuildMemberExpressionError> {
    use BuildMemberExpressionError::*;
    use Rule::{
        member, member_call, member_call_postfix, member_dot,
        member_infix_operator, member_postfix_operator, member_segment,
    };

    let rule = pair.as_rule();

    if rule != member {
        return Err(RuleIsNotAMember(rule));
    };

    let expression = MEMBER_PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            member_segment => {
                let inner_operand =
                    primary.into_inner().next().ok_or(EmptyOperand)?;

                build_ast_expression(inner_operand)
                    .map_err(|e| BuildSegmentExpressionError(e.to_string()))
            }
            unknown_rule => Err(UnexpectedRuleInsteadOfSegment(unknown_rule)),
        })
        .map_infix(|lhs, op, rhs| {
            let lhs = lhs?;
            let rhs = rhs?;

            let member_infix_operator = op.as_rule() else {
                return Err(UnexpectedMemberInfixOperator(op.as_rule()));
            };

            let Some(op) = op.into_inner().next() else {
                return Err(EmptyInfixOperator);
            };

            match op.as_rule() {
                member_dot => Ok((Member::synthetic(lhs, rhs)).into()),
                member_call => {
                    let mut function_call_builder = FunctionCall::builder(lhs);

                    // the inner rule inside member_call is call_arguments, so
                    // we can directly parse it into a function call
                    let argument_rules =
                        op.into_inner().next().ok_or(EmptyInfixOperator)?;

                    // call_arguments contains individual expressions
                    for expression_pair in argument_rules.into_inner() {
                        function_call_builder.add_argument(
                            build_ast_expression(expression_pair).map_err(
                                |e| BuildInnerExpressionError(e.to_string()),
                            )?,
                        );
                    }

                    let function_call = function_call_builder.build();

                    Ok(Member::synthetic(function_call.into(), rhs).into())
                }
                unknown_rule => {
                    return Err(UnexpectedMemberInfixOperator(unknown_rule));
                }
            }
        })
        .map_postfix(|lhs, op| {
            let lhs = lhs?;

            let member_postfix_operator = op.as_rule() else {
                return Err(UnexpectedMemberPostfixOperator(op.as_rule()));
            };

            let Some(op) = op.into_inner().next() else {
                return Err(EmptyPostfixOperator);
            };

            match op.as_rule() {
                member_call_postfix => {
                    let mut function_call_builder = FunctionCall::builder(lhs);

                    // the inner rule inside member_call is call_arguments, so
                    // we can directly parse it into a function call
                    let argument_rules =
                        op.into_inner().next().ok_or(EmptyPostfixOperator)?;

                    // call_arguments contains individual expressions
                    for expression_pair in argument_rules.into_inner() {
                        function_call_builder.add_argument(
                            build_ast_expression(expression_pair).map_err(
                                |e| BuildInnerExpressionError(e.to_string()),
                            )?,
                        );
                    }

                    Ok(function_call_builder.build().into())
                }
                unknown_rule => {
                    return Err(UnexpectedMemberPostfixOperator(unknown_rule));
                }
            }
        })
        .parse(pair.into_inner());

    match expression? {
        Expression::Member(mem) => Ok(mem),
        _ => Err(PrattParserDidNotReturnMember),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildMemberExpressionError {
    /// The first rule is not a member expression.
    #[error("Expected a member expression, but found rule: {0:?}")]
    RuleIsNotAMember(Rule),

    #[error(
        "Expected an inner operand in the member expression, but found none."
    )]
    EmptyOperand,

    #[error("Pratt parser did not return a member expression.")]
    PrattParserDidNotReturnMember,

    #[error("An error occurred while building a segment expression: {0}")]
    BuildSegmentExpressionError(String),

    #[error("An error occurred while building an inner expression: {0}")]
    BuildInnerExpressionError(String),

    #[error("Expected a segment, but found rule: {0:?}")]
    UnexpectedRuleInsteadOfSegment(Rule),

    #[error("Expected a member infix operator, but found rule: {0:?}")]
    UnexpectedMemberInfixOperator(Rule),

    #[error(
        "Expected an infix operator to have an inner rule, but found none."
    )]
    EmptyInfixOperator,

    #[error("Expected a member postfix operator, but found rule: {0:?}")]
    UnexpectedMemberPostfixOperator(Rule),

    #[error(
        "Expected a postfix operator to have an inner rule, but found none."
    )]
    EmptyPostfixOperator,
}
