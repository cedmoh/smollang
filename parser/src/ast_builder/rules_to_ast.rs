use crate::{
    ast_builder::{BuildAstProgramError, build_ast_program},
    rule_parser::Rule,
};
use ast::Program;
use pest::iterators::Pairs;
use thiserror::Error;

/// Converts a sequence of rules into an abstract syntax tree (AST).
pub fn rules_to_ast(rules: Pairs<Rule>) -> Result<Program, RulesToAstError> {
    use RulesToAstError::*;

    build_ast_program(rules).map_err(|error| ProgramRuleToAstError(error))
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum RulesToAstError {
    /// An error occurred while building an expression within the block.
    #[error("An error occurred while building the program AST: {0}")]
    ProgramRuleToAstError(BuildAstProgramError),
}
