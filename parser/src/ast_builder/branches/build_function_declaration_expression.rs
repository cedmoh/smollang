use crate::{
    ast_builder::{
        BuildAstExpressionError, BuildIdentifierExpressionError,
        build_ast_expression, build_identifier_expression,
    },
    rule_parser::Rule,
};
use ast::{FunctionDeclaration, FunctionParameter};
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed declaration expression into an AST
/// representation.
///
/// # Examples
///
/// Empty unnamed function declaration without parameters or body:
///
/// ```pest
/// function_declaration
///  - function_arguments: "||"
/// ```
///
/// Empty named function declaration without parameters or body:
///
/// ```pest
/// function_declaration
///    - function_name > identifier: "x"
///    - function_arguments: "||"
/// ```
///
/// Function declaration without parameters but with body:
///
/// ```pest
/// function_declaration
///     - function_name > identifier: "print"
///     - function_arguments: "||"
///     - function_body > expression > ...
/// ```
///
/// Function declaration with arguments and body:
///
/// ```pest
/// function_declaration
///     - function_name > identifier: "add"
///     - function_arguments
///       - function_argument > identifier: "x"
///       - function_argument > identifier: "y"
///     - function_body > expression > ...
/// ```
pub fn build_function_declaration_expression(
    pair: Pair<Rule>,
) -> Result<FunctionDeclaration, BuildFunctionDeclarationExpressionError> {
    use BuildFunctionDeclarationExpressionError::*;
    use Rule::{
        function_arguments, function_body, function_declaration, function_name,
    };

    let rule = pair.as_rule();

    if rule != function_declaration {
        return Err(RuleIsNotAFunctionDeclaration(rule));
    }

    let mut inner = pair.into_inner();
    let mut builder = FunctionDeclaration::builder();

    let first_pair = inner.next().ok_or(MissingArguments)?;

    let arguments_pair = if first_pair.as_rule() == function_name {
        let identifier_pair =
            first_pair.into_inner().next().ok_or(EmptyFunctionName)?;

        let identifier = build_identifier_expression(identifier_pair)?;
        builder.name(identifier);

        inner.next().ok_or(MissingArguments)?
    } else {
        first_pair
    };

    if arguments_pair.as_rule() != function_arguments {
        return Err(InvalidArgumentsRule(arguments_pair.as_rule()));
    }

    for parameter_pair in arguments_pair.into_inner() {
        let identifier_pair = parameter_pair
            .into_inner()
            .next()
            .ok_or(EmptyFunctionArgument)?;

        let identifier = build_identifier_expression(identifier_pair)?;
        builder.add_param(FunctionParameter::new(identifier));
    }

    if let Some(next_pair) = inner.next() {
        if next_pair.as_rule() != function_body {
            return Err(InvalidBodyRule(next_pair.as_rule()));
        }

        let body_pair =
            next_pair.into_inner().next().ok_or(EmptyFunctionBody)?;
        let body = build_ast_expression(body_pair)?;
        builder.body(body);

        if let Some(extra_pair) = inner.next() {
            return Err(UnexpectedRule(extra_pair.as_rule()));
        }
    }

    Ok(builder.build())
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildFunctionDeclarationExpressionError {
    /// The first rule is not a function declaration.
    #[error(
        "Expected a function declaration expression, but found rule: {0:?}"
    )]
    RuleIsNotAFunctionDeclaration(Rule),

    /// The function declaration is missing its argument list.
    #[error("Missing function arguments in function declaration")]
    MissingArguments,

    /// The arguments rule has an invalid type.
    #[error("Expected function_arguments, but found rule: {0:?}")]
    InvalidArgumentsRule(Rule),

    /// A function argument does not contain an identifier.
    #[error("Function argument is empty")]
    EmptyFunctionArgument,

    /// The function name is empty.
    #[error("Function name is empty")]
    EmptyFunctionName,

    /// The body rule has an invalid type.
    #[error("Expected function_body, but found rule: {0:?}")]
    InvalidBodyRule(Rule),

    /// The function body is empty.
    #[error("Function body is empty")]
    EmptyFunctionBody,

    /// An unexpected extra rule was found in the function declaration.
    #[error("Unexpected rule in function declaration: {0:?}")]
    UnexpectedRule(Rule),

    /// Building the function name failed.
    #[error("Failed to build function name: {0}")]
    BuildIdentifierExpressionError(#[from] BuildIdentifierExpressionError),

    /// Building the function body failed.
    #[error("Failed to build function body expression: {0}")]
    BuildAstExpressionError(#[from] BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;

    #[test]
    fn should_build_function_declaration_with_name_arguments_and_body() {
        // Arrange
        let input = r#"add |x, y| x"#;

        let function_rule =
            parse_string_to_rule(input, Rule::function_declaration)
                .expect("Expected input to be parsed to rules successfully.")
                .next()
                .expect("Expected input to contain a function declaration.");

        // Act
        let result = build_function_declaration_expression(function_rule);

        // Assert
        assert!(result.is_ok());
        let function = result.unwrap();
        assert!(function.name.is_some());
        assert_eq!(function.params.items.len(), 2);
        assert!(function.body.is_some());
    }

    #[test]
    fn should_build_function_declaration_without_name_or_body() {
        // Arrange
        let input = r#"||"#;

        let function_rule =
            parse_string_to_rule(input, Rule::function_declaration)
                .expect("Expected input to be parsed to rules successfully.")
                .next()
                .expect("Expected input to contain a function declaration.");

        // Act
        let result = build_function_declaration_expression(function_rule);

        // Assert
        assert!(result.is_ok());
        let function = result.unwrap();
        assert!(function.name.is_none());
        assert_eq!(function.params.items.len(), 0);
        assert!(function.body.is_none());
    }

    #[test]
    fn should_return_error_when_rule_is_not_function_declaration() {
        // Arrange
        let input = "x";

        let identifier_rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected input to contain an identifier.");

        // Act
        let result = build_function_declaration_expression(identifier_rule);

        // Assert
        assert!(matches!(
            result,
            Err(BuildFunctionDeclarationExpressionError::RuleIsNotAFunctionDeclaration(_))
        ));
    }
}
