use crate::Expressions;

/// A pipe expression, which allows for chaining multiple expressions together.
///
/// # Example
///
/// ```
/// 1 |> add(it, 2) |> multiply(it, 3)
/// ```
pub struct Pipe {
    pub expressions: Expressions,
}
