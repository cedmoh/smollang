use crate::{Identifier, Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    /// The path of the module to import.
    pub path: String,

    /// The identifiers to import from the module.
    pub imports: Vec<Identifier>,

    /// The location of the AST node in the source code.
    pub span: Span,
}

impl Use {
    pub fn new(path: String, imports: Vec<Identifier>, span: Span) -> Self {
        Self {
            path,
            imports,
            span,
        }
    }

    pub fn synthetic(path: String, imports: Vec<Identifier>) -> Self {
        Self::new(path, imports, Span::DUMMY)
    }

    pub fn builder(path: String) -> UseBuilder {
        UseBuilder::new(path)
    }
}

#[derive(Debug, Default)]
pub struct UseBuilder {
    pub path: String,
    pub imports: Vec<Identifier>,
}

impl UseBuilder {
    pub fn new(path: String) -> Self {
        Self {
            path,
            imports: Vec::new(),
        }
    }

    pub fn add_import(&mut self, import: Identifier) -> &mut Self {
        self.imports.push(import);
        self
    }

    pub fn build(self) -> Use {
        Use::synthetic(self.path, self.imports)
    }
}
