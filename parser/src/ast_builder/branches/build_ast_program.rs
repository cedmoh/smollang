use std::fmt::Display;

use ast::{Expressions, Program};
use pest::iterators::Pairs;

use crate::{
    ast_builder::{branches::build_ast_expression::BuildAstExpressionError, build_ast_expression},
    rules::{self, Rule},
};

/// Converts the pest rules of a parsed program into an AST representation.
///
/// ```
/// - program
/// - expression > ...
/// - expression > ...
/// - EOI: ""
/// ```
pub fn build_ast_program(mut pairs: Pairs<Rule>) -> Result<Program, BuildAstProgramError> {
    let next_node = pairs.next();

    let Some(program) = next_node else {
        return Err(BuildAstProgramError::EmptyProgram);
    };

    let rule = program.as_rule();
    if rule != rules::Rule::program {
        return Err(BuildAstProgramError::FirstRuleIsNotProgram(rule));
    };

    let mut expressions_vec = Vec::new();

    for maybe_expression in program.into_inner() {
        // If we encounter the end of input, we can stop processing further expressions.
        if maybe_expression.as_rule() == rules::Rule::EOI {
            break;
        }

        let ast_expression = build_ast_expression(maybe_expression)
            .map_err(BuildAstProgramError::BuildAstExpressionError)?;

        expressions_vec.push(ast_expression);
    }

    Ok(Program::new(Expressions::new(expressions_vec)))
}

#[derive(Debug, PartialEq)]
pub enum BuildAstProgramError {
    /// The program is empty.
    EmptyProgram,

    /// The first rule is not a program.
    FirstRuleIsNotProgram(Rule),

    /// An error occurred while building an expression.
    BuildAstExpressionError(BuildAstExpressionError),
}

impl Display for BuildAstProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildAstProgramError::EmptyProgram => write!(f, "The program is empty."),
            BuildAstProgramError::FirstRuleIsNotProgram(rule) => {
                write!(
                    f,
                    "Expected first rule to be a program. Found rule: {:?}",
                    rule
                )
            }
            BuildAstProgramError::BuildAstExpressionError(error) => {
                write!(
                    f,
                    "An error occurred while building an expression > {}",
                    error
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::parse_to_rules;
    use ast::{Expression, IntegerLiteral, Literal};

    #[test]
    fn should_build_empty_program() {
        // Arrange
        let input = r#""#;

        let rules = parse_to_rules(input).expect("Expected to parse rules successfully.");

        // Act
        let ast_program = build_ast_program(rules);

        // Assert
        assert_eq!(ast_program, Ok(Program::default()))
    }

    #[test]
    fn should_build_program_with_a_literal_expression() {
        // Arrange
        let input = r#"69"#;

        let rules = parse_to_rules(input).expect("Expected to parse rules successfully.");

        // Act
        let ast_program = build_ast_program(rules);

        // Assert
        let expected_number = 69;

        assert_eq!(
            ast_program,
            Ok(Program::new(Expressions::new(vec![Expression::Literal(
                Literal::Integer(IntegerLiteral::new(expected_number))
            )])))
        )
    }
}
