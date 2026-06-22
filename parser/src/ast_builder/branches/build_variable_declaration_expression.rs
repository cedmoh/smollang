use crate::{
    ast_builder::{
        BuildAstExpressionError, BuildIdentifierExpressionError,
        build_ast_expression,
    },
    into_ast_span::IntoAstSpan,
    rule_parser::Rule,
};
use ast::{Identifier, VariableDeclaration};
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed declaration expression into an AST
/// representation.
///
/// # Examples
///
/// Mutable variable declaration without initial value:
///
/// ```pest
/// variable_declaration
///  - variable_name: "y"
///  - var: "var"
/// ```
///
/// Mutable variable declaration with initial value:
///
/// ```pest
/// variable_declaration
///  - variable_name: "x"
///  - var: "var"
///  - variable_initialization > expression > ...
/// ```
///
/// Immutable variable declaration with initial value:
///
/// ```pest
/// variable_declaration
///   - variable_name: "x"
///   - val: "val"
///   - variable_initialization > expression > ...
/// ```
pub fn build_variable_declaration_expression(
    pair: Pair<Rule>,
) -> Result<VariableDeclaration, BuildVariableDeclarationExpressionError> {
    use BuildVariableDeclarationExpressionError::*;
    use Rule::{
        val, var, variable_declaration, variable_initialization, variable_name,
    };

    let rule = pair.as_rule();

    if rule != variable_declaration {
        return Err(RuleIsNotAVariableDeclaration(rule));
    }

    let span = pair.as_span().into_ast_span();

    let mut inner = pair.into_inner();

    let name_pair = inner.next().ok_or(MissingName)?;
    if name_pair.as_rule() != variable_name {
        return Err(InvalidNameRule(name_pair.as_rule()));
    }

    let name_str = name_pair.as_str();
    if name_str.is_empty() {
        return Err(EmptyName);
    }
    let name = Identifier::new(
        name_str.to_string(),
        name_pair.as_span().into_ast_span(),
    );

    let mutability_pair = inner.next().ok_or(MissingMutability)?;
    let mutability_rule = mutability_pair.as_rule();
    let is_mutable = if mutability_rule == var {
        true
    } else if mutability_rule == val {
        false
    } else {
        return Err(InvalidMutabilityToken(mutability_rule));
    };

    let initial_value = match inner.next() {
        Some(init_pair) => {
            if init_pair.as_rule() != variable_initialization {
                return Err(InvalidInitializationRule(init_pair.as_rule()));
            }

            let expression_pair =
                init_pair.into_inner().next().ok_or(MissingInitialization)?;
            let initial_value = build_ast_expression(expression_pair)?;

            Some(Box::new(initial_value))
        }
        None => None,
    };

    Ok(VariableDeclaration::new(
        name,
        is_mutable,
        initial_value,
        span,
    ))
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildVariableDeclarationExpressionError {
    /// The first rule is not a variable declaration.
    #[error(
        "Expected a variable declaration expression, but found rule: {0:?}"
    )]
    RuleIsNotAVariableDeclaration(Rule),

    /// The variable declaration is missing its name.
    #[error("Missing variable name in variable declaration")]
    MissingName,

    /// The name rule is not variable_name.
    #[error("Expected variable_name, but found rule: {0:?}")]
    InvalidNameRule(Rule),

    /// The variable_name does not contain an identifier.
    #[error("Variable name is empty")]
    EmptyName,

    /// The variable declaration is missing its mutability.
    #[error("Missing mutability in variable declaration")]
    MissingMutability,

    /// The mutability token is not var or val.
    #[error("Expected var or val, but found rule: {0:?}")]
    InvalidMutabilityToken(Rule),

    /// The variable declaration is missing its initialization.
    #[error("Missing initialization in variable declaration")]
    MissingInitialization,

    /// The initialization rule is not variable_initialization.
    #[error("Expected variable_initialization, but found rule: {0:?}")]
    InvalidInitializationRule(Rule),

    /// Building the identifier failed.
    #[error("Failed to build variable name: {0}")]
    BuildIdentifierExpressionError(#[from] BuildIdentifierExpressionError),

    /// Building the initial value expression failed.
    #[error("Failed to build initial value expression: {0}")]
    BuildAstExpressionError(#[from] BuildAstExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::Span;

    #[test]
    fn should_return_error_when_rule_is_not_variable_declaration() {
        // Arrange
        let input = r#"x"#;
        let rule = parse_string_to_rule(input, Rule::identifier)
            .expect("Failed to parse input")
            .next()
            .expect("Expected identifier rule");

        // Act
        let result = build_variable_declaration_expression(rule);

        // Assert
        assert!(matches!(
            result,
            Err(BuildVariableDeclarationExpressionError::RuleIsNotAVariableDeclaration(_))
        ));
    }

    #[test]
    fn should_build_mutable_declaration_without_initial_value_when_given_var_without_initialization()
     {
        // Arrange
        let input = "y var";
        let rule = parse_string_to_rule(input, Rule::variable_declaration)
            .expect("Failed to parse input")
            .next()
            .expect("Expected variable_declaration rule");

        // Act
        let result = build_variable_declaration_expression(rule);

        // Assert
        assert!(result.is_ok());
        let declaration = result.unwrap();
        assert_eq!(declaration.name.id, "y");
        assert_ne!(declaration.name.span, Span::DUMMY);
        assert_ne!(declaration.span, Span::DUMMY);
        assert!(declaration.is_mutable);
        assert!(declaration.initial_value.is_none());
    }

    #[test]
    fn should_build_mutable_declaration_with_initial_value_when_given_var_with_initialization()
     {
        // Arrange
        let input = "x var y";
        let rule = parse_string_to_rule(input, Rule::variable_declaration)
            .expect("Failed to parse input")
            .next()
            .expect("Expected variable_declaration rule");

        // Act
        let result = build_variable_declaration_expression(rule);

        // Assert
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        let declaration = result.unwrap();
        assert_eq!(declaration.name.id, "x");
        assert_ne!(declaration.name.span, Span::DUMMY);
        assert_ne!(declaration.span, Span::DUMMY);
        assert!(declaration.is_mutable);
        assert!(declaration.initial_value.is_some());
    }

    #[test]
    fn should_build_immutable_declaration_when_given_val_with_initialization() {
        // Arrange
        let input = "x val y";
        let rule = parse_string_to_rule(input, Rule::variable_declaration)
            .expect("Failed to parse input")
            .next()
            .expect("Expected variable_declaration rule");

        // Act
        let result = build_variable_declaration_expression(rule);

        // Assert
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
        let declaration = result.unwrap();
        assert_eq!(declaration.name.id, "x");
        assert_ne!(declaration.name.span, Span::DUMMY);
        assert_ne!(declaration.span, Span::DUMMY);
        assert!(!declaration.is_mutable);
        assert!(declaration.initial_value.is_some());
    }
}
