use std::fmt::Display;

use crate::rules::Rule;
use ast::Expression;
use pest::iterators::Pair;

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
        Rule::block => todo!(),
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

#[derive(Debug, PartialEq)]
pub enum BuildAstExpressionError {
    /// The expression is empty.
    EmptyExpression,

    /// The first rule is not an expression.
    RuleIsNotAnExpression(Rule),

    /// The expression is not recognized.
    UnrecognizedExpression(Rule),
}

impl Display for BuildAstExpressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildAstExpressionError::EmptyExpression => write!(f, "The expression is empty."),
            BuildAstExpressionError::RuleIsNotAnExpression(rule) => {
                write!(f, "Rule is not an expression. Found rule: {:?}", rule)
            }
            BuildAstExpressionError::UnrecognizedExpression(rule) => {
                write!(f, "Unrecognized expression. Found rule: {:?}", rule)
            }
        }
    }
}
