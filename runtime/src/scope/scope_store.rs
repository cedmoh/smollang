use std::collections::HashMap;

use nanoid::nanoid;

use super::{ScopeId, ScopeItem};

/// A store for items in a scope, such as variables and functions.
#[derive(Debug, Default)]
pub struct ScopeStore {
    store: HashMap<ScopeId, ScopeItem>,
}

impl ScopeStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_item(
        &mut self,
        scope_id: ScopeId,
        item: ScopeItem,
    ) -> Result<(), AddScopeItemError> {
        if self.store.contains_key(&scope_id) {
            return Err(AddScopeItemError::ScopeItemAlreadyExists);
        }

        self.store.insert(scope_id, item);

        Ok(())
    }

    pub fn add_anonymous_item(&mut self, item: ScopeItem) -> Result<ScopeId, AddScopeItemError> {
        // TODO: Find a better way to generate unique IDs for anonymous items.
        let unique_id = nanoid!(8);

        let scope_id = ScopeId::anonymous(&unique_id);

        self.add_item(scope_id.clone(), item)?;

        Ok(scope_id)
    }

    pub fn get_item(&self, scope_id: &ScopeId) -> Option<&ScopeItem> {
        self.store.get(scope_id)
    }

    pub fn get_item_mut(&mut self, scope_id: &ScopeId) -> Option<&mut ScopeItem> {
        self.store.get_mut(scope_id)
    }
}

#[derive(Debug)]
pub enum AddScopeItemError {
    /// A scope item with the same name already exists in the current scope.
    ScopeItemAlreadyExists,
}
