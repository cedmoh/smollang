use pest::pratt_parser::{Assoc, Op, PrattParser};
use std::sync::LazyLock;

use crate::rule_parser::rule_parser::Rule;

/// A Pratt parser for member expressions, which are left-associative and can
/// have both infix and postfix operators.
pub static MEMBER_PRATT_PARSER: LazyLock<PrattParser<Rule>> =
    LazyLock::new(|| {
        use Assoc::Left;
        PrattParser::new()
            // Weakest
            .op(Op::infix(Rule::member_infix_operator, Left))
            .op(Op::postfix(Rule::member_postfix_operator))
        // Strongest
    });
