#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeId {
    identifier: String,
}

impl ScopeId {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }

    pub fn anonymous(unique_id: &str) -> Self {
        Self {
            identifier: format!("<anonymous:{}>", unique_id),
        }
    }
}
