pub trait IntoAstSpan {
    fn into_ast_span(self) -> ast::Span;
}

impl IntoAstSpan for pest::Span<'_> {
    fn into_ast_span(self) -> ast::Span {
        ast::Span::new(self.start(), self.end())
    }
}
