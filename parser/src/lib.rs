mod ast_builder;
mod parse_string_to_program_ast;
mod rule_parser;

pub use parse_string_to_program_ast::{
    ParseProgramError, parse_string_to_program_ast,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_block_expression() {
        // Arrange
        let input = r#"hello"#;

        // Act
        let ast = parse_string_to_program_ast(input);

        println!("{:#?}", ast);

        // Assert
        assert!(ast.is_ok());
    }
}
