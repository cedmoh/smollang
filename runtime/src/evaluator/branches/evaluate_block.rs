use ast::Block;

use crate::{EvaluationResult, Evaluator, Scope, evaluate_expressions_and_return_last_value};

/// Evaluate a block by evaluating each expression in the block in order and returning the value of the last expression.
pub fn evaluate_block(block: Block, evaluator: &Evaluator, scope: &mut Scope) -> EvaluationResult {
    let expressions = block.body.items;
    evaluate_expressions_and_return_last_value(expressions, evaluator, scope)
}
