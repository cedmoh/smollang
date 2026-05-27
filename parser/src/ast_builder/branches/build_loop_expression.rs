use ast::Loop;
use pest::iterators::Pair;
use thiserror::Error;

use crate::rule_parser::Rule;

pub fn build_loop_expression(
    _pair: Pair<'_, Rule>,
) -> Result<Loop, BuildLoopExpressionError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum BuildLoopExpressionError {}
