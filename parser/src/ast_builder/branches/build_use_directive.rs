use crate::{
    ast_builder::{
        BuildIdentifierExpressionError, build_identifier_expression,
    },
    rule_parser::Rule,
};
use ast::Use;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed use directive into an AST
/// representation.
///
/// # Examples
///
/// ```pest
/// - use_directive
///   - use_imports
///     - identifier: "a"
///     - identifier: "b"
///   - use_path
///     - identifier | string_literal
/// ```
pub fn build_use_directive(
    pair: Pair<Rule>,
) -> Result<Use, BuildUseDirectiveError> {
    use BuildUseDirectiveError::*;
    use Rule::{
        identifier, string_literal, use_directive, use_imports, use_path,
    };

    if pair.as_rule() != use_directive {
        return Err(RuleIsNotAUseDirective(pair.as_rule()));
    }

    let mut inner = pair.into_inner();

    let imports_pair = inner.next().ok_or(MissingImports)?;
    if imports_pair.as_rule() != use_imports {
        return Err(InvalidImportsRule(imports_pair.as_rule()));
    }

    let mut imports = Vec::new();
    for import_pair in imports_pair.into_inner() {
        if import_pair.as_rule() != identifier {
            return Err(InvalidImportRule(import_pair.as_rule()));
        }

        let import_identifier = build_identifier_expression(import_pair)?;
        imports.push(import_identifier);
    }

    let path_pair = inner.next().ok_or(MissingPath)?;
    if path_pair.as_rule() != use_path {
        return Err(InvalidPathRule(path_pair.as_rule()));
    }

    let path_node = path_pair.into_inner().next().ok_or(EmptyPath)?;
    let path = match path_node.as_rule() {
        identifier => path_node.as_str().to_string(),
        string_literal => {
            let raw = path_node.as_str();
            raw.trim_matches('\'').to_string()
        }
        other => return Err(InvalidPathValueRule(other)),
    };

    let mut use_builder = Use::builder(path);
    for import in imports {
        use_builder.add_import(import);
    }

    Ok(use_builder.build())
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildUseDirectiveError {
    /// The rule is not a use_directive.
    #[error("Expected a use directive, but found rule: {0:?}")]
    RuleIsNotAUseDirective(Rule),

    /// The use directive is missing imports.
    #[error("Missing imports in use directive")]
    MissingImports,

    /// The imports node has the wrong rule.
    #[error("Expected use_imports, but found rule: {0:?}")]
    InvalidImportsRule(Rule),

    /// One import entry has the wrong rule.
    #[error("Expected identifier in use imports, but found rule: {0:?}")]
    InvalidImportRule(Rule),

    /// The use directive is missing a path.
    #[error("Missing path in use directive")]
    MissingPath,

    /// The path node has the wrong wrapper rule.
    #[error("Expected use_path, but found rule: {0:?}")]
    InvalidPathRule(Rule),

    /// The path wrapper has no inner value.
    #[error("Use path is empty")]
    EmptyPath,

    /// The path value rule is unsupported.
    #[error(
        "Expected identifier or string_literal in use path, but found rule: {0:?}"
    )]
    InvalidPathValueRule(Rule),

    /// Building an imported identifier failed.
    #[error("An error occurred while building an imported identifier: {0}")]
    BuildIdentifierExpressionError(#[from] BuildIdentifierExpressionError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::Identifier;

    #[test]
    fn should_build_use_directive_with_identifier_path() {
        // Arrange
        let input = r#"use first, second from std"#;

        let use_pair = parse_string_to_rule(input, Rule::use_directive)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected at least one use_directive rule.");

        // Act
        let ast_use = build_use_directive(use_pair);

        // Assert
        let expected = Use::new(
            "std".to_string(),
            vec![
                Identifier::new("first".to_string()),
                Identifier::new("second".to_string()),
            ],
        );

        assert_eq!(ast_use, Ok(expected));
    }

    #[test]
    fn should_build_use_directive_with_string_path() {
        // Arrange
        let input = r#"use first from 'pkg/core'"#;

        let use_pair = parse_string_to_rule(input, Rule::use_directive)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected at least one use_directive rule.");

        // Act
        let ast_use = build_use_directive(use_pair);

        // Assert
        let expected = Use::new(
            "pkg/core".to_string(),
            vec![Identifier::new("first".to_string())],
        );

        assert_eq!(ast_use, Ok(expected));
    }

    #[test]
    fn should_return_error_when_rule_is_not_use_directive() {
        // Arrange
        let input = r#"identifier"#;

        let identifier_pair = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected at least one identifier rule.");

        // Act
        let ast_use = build_use_directive(identifier_pair);

        // Assert
        assert!(matches!(
            ast_use,
            Err(BuildUseDirectiveError::RuleIsNotAUseDirective(_))
        ));
    }
}
