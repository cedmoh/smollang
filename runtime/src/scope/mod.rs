mod scope_id;
mod scope_item;
mod scope_store;

pub use scope_id::*;
pub use scope_item::*;
pub use scope_store::*;

use crate::Value;

#[derive(Debug, Default)]
pub struct Scope {
    /// The items defined in the scope, such as variables and functions.
    items: ScopeStore,

    /// The parent scope, if any.
    parent: Option<Box<Scope>>,
}

/// A locator for an item in the scope, such as a variable or function name.
/// TODO: This is currently just a string representing the name of the item,
/// but it should be extended to support anonymous items.
pub type ScopeItemLocator<'a> = &'a str;

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parent(parent: Scope) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            ..Default::default()
        }
    }

    /// Add a variable to the current scope.
    pub fn add_variable(
        &mut self,
        locator: ScopeItemLocator,
        value: Value,
        is_mutable: bool,
    ) -> Result<(), AddScopeItemError> {
        self.items.add_item(
            ScopeId::new(locator.to_string()),
            ScopeItem::Variable { value, is_mutable },
        )
    }

    /// Look up a variable or function in the current scope and its parent scopes.
    pub fn lookup(&self, locator: ScopeItemLocator) -> Option<&ScopeItem> {
        let scope_id = ScopeId::new(locator.to_string());

        if let Some(item) = self.items.get_item(&scope_id) {
            return Some(item);
        }

        if let Some(parent) = &self.parent {
            return parent.lookup(locator);
        }

        None
    }

    /// Look up a variable or function in the current scope only.
    pub fn lookup_current_scope(&self, locator: ScopeItemLocator) -> Option<&ScopeItem> {
        let scope_id = ScopeId::new(locator.to_string());

        self.items.get_item(&scope_id)
    }

    /// Assign a new value to an existing variable in the current scope or its parent scopes.
    /// Returns an error if the variable is immutable or does not exist.
    pub fn assign_variable(
        &mut self,
        locator: ScopeItemLocator,
        value: Value,
    ) -> Result<(), AssignVariableError> {
        let scope_id = ScopeId::new(locator.to_string());

        if let Some(item) = self.items.get_item_mut(&scope_id) {
            return match item {
                ScopeItem::Variable {
                    value: existing_value,
                    is_mutable: true,
                } => {
                    *existing_value = value;
                    Ok(())
                }
                ScopeItem::Variable {
                    is_mutable: false, ..
                } => Err(AssignVariableError::VariableIsImmutable),
                ScopeItem::Function => Err(AssignVariableError::ItemIsNotAVariable),
            };
        }

        if let Some(parent) = &mut self.parent {
            return parent.assign_variable(locator, value);
        }

        // Variable does not exist in the current scope or any parent scopes.
        Err(AssignVariableError::VariableIsNotDefined)
    }
}

#[derive(Debug)]
enum AssignVariableError {
    /// The variable is immutable and cannot be assigned to.
    VariableIsImmutable,

    /// The variable does not exist in the current scope or any parent scopes.
    VariableIsNotDefined,

    /// The item with the same name exists but is not a variable.
    ItemIsNotAVariable,
}
