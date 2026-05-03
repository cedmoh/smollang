use crate::{AddScopeItemError, Scope, Value};

/// The environment in which the program is executed, containing global variables and functions.
#[derive(Debug, Default)]
pub struct Environment {
    pub global_scope: Scope,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a global variable to the environment.
    /// Global variables are immutable.
    pub fn add_global_variable(
        &mut self,
        identifier: &str,
        value: Value,
    ) -> Result<(), AddGlobalVariableError> {
        self.global_scope
            .add_variable(
                identifier, value, // Global variables are immutable by default.
                false,
            )
            .map_err(|error| AddGlobalVariableError::AddScopeItemError(error))
    }
}

#[derive(Debug)]
pub enum AddGlobalVariableError {
    AddScopeItemError(AddScopeItemError),
}
