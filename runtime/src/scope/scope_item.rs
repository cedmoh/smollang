use crate::Value;

/// The different types of items that can be stored in a scope.
#[derive(Debug)]
pub enum ScopeItem {
    Variable { value: Value, is_mutable: bool },
    Function,
}
