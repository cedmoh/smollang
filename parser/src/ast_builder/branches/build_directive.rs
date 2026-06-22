use crate::{
    ast_builder::{BuildUseDirectiveError, build_use_directive},
    rule_parser::Rule,
};
use ast::Directive;
use pest::iterators::Pair;
use thiserror::Error;

/// Converts the pest rules of a parsed directive into an AST representation.
///
/// # Examples
///
/// ```pest
/// - directive
///   - use_directive
/// ```
pub fn build_ast_directive(
    pair: Pair<Rule>,
) -> Result<Directive, BuildAstDirectiveError> {
    use BuildAstDirectiveError::*;
    use Directive::Use;
    use Rule::{directive, use_directive};

    match pair.as_rule() {
        directive => {
            let mut inner = pair.into_inner();
            let Some(inner_directive) = inner.next() else {
                return Err(EmptyDirective);
            };

            match inner_directive.as_rule() {
                use_directive => build_use_directive(inner_directive)
                    .map(Use)
                    .map_err(Into::into),
                other => Err(UnrecognizedDirective(other)),
            }
        }
        use_directive => build_use_directive(pair).map(Use).map_err(Into::into),
        other => Err(UnrecognizedDirective(other)),
    }
}

#[derive(Debug, PartialEq, Error)]
#[non_exhaustive]
pub enum BuildAstDirectiveError {
    /// The directive rule does not contain an inner directive.
    #[error("The directive is empty.")]
    EmptyDirective,

    /// The directive variant is unrecognized.
    #[error("Unrecognized directive. Found rule : {0:?}")]
    UnrecognizedDirective(Rule),

    /// Building a use directive failed.
    #[error("An error occurred while building a use directive: {0}")]
    BuildUseDirectiveError(#[from] BuildUseDirectiveError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_parser::parse_string_to_rule;
    use ast::{Identifier, Use};

    #[test]
    fn should_build_use_directive_when_rule_is_directive() {
        // Arrange
        let input = r#"use first, second from std"#;

        let directive_pair = parse_string_to_rule(input, Rule::directive)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected at least one directive rule.");

        // Act
        let ast_directive = build_ast_directive(directive_pair);

        // Assert
        let expected = Directive::Use(Use::synthetic(
            "std".to_string(),
            vec![
                Identifier::synthetic("first".to_string()),
                Identifier::synthetic("second".to_string()),
            ],
        ));

        assert_eq!(ast_directive, Ok(expected));
    }

    #[test]
    fn should_return_error_when_rule_is_not_directive() {
        // Arrange
        let input = r#"identifier"#;

        let identifier_pair = parse_string_to_rule(input, Rule::identifier)
            .expect("Expected input to be parsed to rules successfully.")
            .next()
            .expect("Expected at least one identifier rule.");

        // Act
        let ast_directive = build_ast_directive(identifier_pair);

        // Assert
        assert!(matches!(
            ast_directive,
            Err(BuildAstDirectiveError::UnrecognizedDirective(_))
        ));
    }
}
