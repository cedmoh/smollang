/// A visitor trait for traversing the AST and applying an operation to it.
pub trait Visitor<T, E> {
    /// Visits a node in the AST and applies an operation to it.
    fn visit(&mut self, program: &T) -> Result<(), E>;
}
