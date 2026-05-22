use ast::Program;
use pest::iterators::Pairs;
use thiserror::Error;

use crate::{
    ast_builder::{
        BuildAstDirectiveError,
        branches::build_ast_expression::BuildAstExpressionError,
        build_ast_directive, build_ast_expression,
    },
    rule_parser::{self, Rule},
};

/// Converts the pest rules of a parsed program into an AST representation.
///
/// # Examples
///
/// ```pest
/// - program
///   - expression > ...
///   - expression > ...
///   - EOI: ""
/// ```
pub fn build_ast_program(
    mut pairs: Pairs<Rule>,
) -> Result<Program, BuildAstProgramError> {
    use BuildAstProgramError::*;
    use Rule::{EOI, directive, expression};

    let next_node = pairs.next();

    let Some(program) = next_node else {
        return Err(EmptyProgram);
    };

    let rule = program.as_rule();
    if rule != rule_parser::Rule::program {
        return Err(FirstRuleIsNotProgram(rule));
    };

    let mut program_builder = Program::builder();

    for program_inner_rule in program.into_inner() {
        match program_inner_rule.as_rule() {
            // If we encounter the end of input, we can stop processing further
            // rules.
            EOI => break,
            directive => {
                let ast_directive = build_ast_directive(program_inner_rule)
                    .map_err(BuildInnerDirectiveError)?;

                program_builder.add_directive(ast_directive);
            }
            expression => {
                let ast_expression = build_ast_expression(program_inner_rule)
                    .map_err(BuildInnerExpressionError)?;

                program_builder.add_expression(ast_expression);
            }
            other => return Err(UnexpectedRuleInProgram(other)),
        }
    }

    Ok(program_builder.build())
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildAstProgramError {
    /// The program is empty.
    #[error("The program is empty. Expected at least one expression.")]
    EmptyProgram,

    /// The first rule is not a program.
    #[error("Expected first rule to be a program. Found rule: {0:?}")]
    FirstRuleIsNotProgram(Rule),

    /// An unexpected rule was encountered in the program.
    #[error("An unexpected rule was encountered in the program: {0:?}")]
    UnexpectedRuleInProgram(Rule),

    /// An error occurred while building an expression.
    #[error("An error occurred while building an inner expression : {0}")]
    BuildInnerExpressionError(BuildAstExpressionError),

    /// An error occurred while building a directive.
    #[error("An error occurred while building an inner directive : {0}")]
    BuildInnerDirectiveError(BuildAstDirectiveError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_program_rule;

    #[test]
    fn should_build_empty_program() {
        // Arrange
        let input = r#""#;

        let rules = parse_string_to_program_rule(input)
            .expect("Expected input to be parsed to rules successfully.");

        // Act
        let ast_program = build_ast_program(rules);

        // Assert
        assert_eq!(ast_program, Ok(Program::default()))
    }
}
