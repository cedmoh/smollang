/// A pattern, which is used in match expressions to specify the structure of the value being matched.
///
/// # Example
///
/// ```
/// [x, y]
/// ```
/// In this example, `[x, y]` is a pattern that matches a tuple with two elements, where the first element is bound to `x` and the second element is bound to `y`.
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    /// The content of the pattern, which can be a literal, an identifier, a tuple, a list, or a wildcard.
    pub content: String,
}
