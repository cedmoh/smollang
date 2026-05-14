use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/rule_parser/.pest"]
pub struct RuleParser;
