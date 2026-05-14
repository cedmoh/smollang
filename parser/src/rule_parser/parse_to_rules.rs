use crate::rule_parser::rule_parser::{Rule, RuleParser};
use pest::{Parser, error::Error as RuleParseError, iterators::Pairs};

/// Parses the input string into a sequence of rules starting from the `program`
/// rule. Used for parsing an entire program.
pub fn parse_string_to_program_rule(
    input: &str,
) -> Result<Pairs<'_, Rule>, RuleParseError<Rule>> {
    parse_string_to_rule(input, Rule::program)
}

/// Parses the input string into a sequence of rules starting from the specified
/// rule. Used for parsing specific expressions or statements.
pub fn parse_string_to_rule(
    input: &str,
    rule: Rule,
) -> Result<Pairs<'_, Rule>, RuleParseError<Rule>> {
    RuleParser::parse(rule, input)
}
