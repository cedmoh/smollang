use crate::rules::rule_parser::{Rule, RuleParser};
use pest::{Parser, error::Error as RuleParseError, iterators::Pairs};

/// Parses the input string into a sequence of rules.
pub fn parse_to_rules(input: &str) -> Result<Pairs<Rule>, RuleParseError<Rule>> {
    RuleParser::parse(Rule::program, input)
}
