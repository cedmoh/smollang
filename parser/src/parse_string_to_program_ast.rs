use ast::Program;
use thiserror::Error;

use crate::{
    ast_builder::{RulesToAstError, rules_to_ast},
    rule_parser::parse_string_to_program_rule,
};

/// This function serves as the main entry point for parsing a Smollang program
/// from a string input. It first converts the input string into a sequence of
/// pest rules, and then transforms those rules into an abstract syntax tree
/// (AST) representation of the program.
pub fn parse_string_to_program_ast(
    input: &str,
) -> Result<Program, ParseProgramError> {
    use ParseProgramError::*;

    let rules = parse_string_to_program_rule(input)
        .map_err(|e| StringToRulesError(e.to_string()))?;

    rules_to_ast(rules).map_err(|e| RulesToAstError(e))
}

#[derive(Debug, PartialEq, Error)]
pub enum ParseProgramError {
    /// An error occurred while parsing the input string into pest rules.
    #[error(
        "An error occurred while parsing the input string into rules : {0}"
    )]
    StringToRulesError(String),

    /// An error occurred while converting the pest rules into an AST.
    #[error("An error occurred while converting rules into an AST : {0}")]
    RulesToAstError(RulesToAstError),
}
