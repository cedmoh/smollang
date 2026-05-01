use pest::pratt_parser::{Assoc, Op, PrattParser};
use std::sync::LazyLock;

use crate::rules::rule_parser::Rule;

pub static PRATT_PARSER: LazyLock<PrattParser<Rule>> = LazyLock::new(|| {
    use Assoc::{Left, Right};

    PrattParser::new()
        // Weakest
        .op(Op::infix(Rule::range, Right) | Op::infix(Rule::range_inclusive, Right))
        .op(Op::infix(Rule::logic_or, Left))
        .op(Op::infix(Rule::logic_and, Left))
        .op(Op::infix(Rule::equals, Left)
            | Op::infix(Rule::not_equals, Left)
            | Op::infix(Rule::less_than, Left)
            | Op::infix(Rule::greater_than, Left)
            | Op::infix(Rule::less_than_or_equals, Left)
            | Op::infix(Rule::greater_than_or_equals, Left))
        .op(Op::infix(Rule::addition, Left) | Op::infix(Rule::subtraction, Left))
        .op(Op::infix(Rule::multiplication, Left)
            | Op::infix(Rule::division, Left)
            | Op::infix(Rule::modulo, Left))
        .op(Op::infix(Rule::exponent, Right))
    // Strongest
});
