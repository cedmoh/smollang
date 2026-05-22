use crate::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    pub path: String,
    pub imports: Vec<Identifier>,
}

impl Use {
    pub fn new(path: String, imports: Vec<Identifier>) -> Self {
        Self { path, imports }
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
        Use::new(self.path, self.imports)
    }
}
