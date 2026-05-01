use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/rules/.pest"]
pub struct RuleParser;
