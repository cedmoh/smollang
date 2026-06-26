#[derive(Debug, Clone)]
pub struct Local {
    /// The name of the local variable.
    pub id: String,

    /// The depth of the local variable in the scope stack.
    /// `None` if the local variable is not yet defined.
    pub depth: Option<usize>,
}
