/// A continue expression, which is used to skip the current iteration of a
/// loop.
///
/// # Examples
///
/// ```smollang
/// cont
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Continue;

impl Continue {
    pub fn new() -> Self {
        Self
    }
}
