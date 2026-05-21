mod ast_implementations;
mod helpers;

pub use helpers::*;

use std::fmt::{self, Display, Formatter};

/// Formats AST nodes using a YAML-like indentation-aware representation.
pub trait PrettyPrint {
    /// Formats this value with the provided indentation level.
    fn fmt_with_indent(
        &self,
        f: &mut Formatter<'_>,
        indent: usize,
    ) -> fmt::Result;

    /// Returns a wrapper that renders this value using [`Display`].
    fn pretty(&self) -> PrettyPrinter<'_, Self>
    where
        Self: Sized,
    {
        PrettyPrinter { value: self }
    }
}

/// A [`Display`] wrapper for any [`PrettyPrint`] implementation.
pub struct PrettyPrinter<'a, T: ?Sized> {
    /// The value being wrapped.
    value: &'a T,
}

impl<'a, T: PrettyPrint + ?Sized> Display for PrettyPrinter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.value.fmt_with_indent(f, 0)
    }
}

impl<T: PrettyPrint> PrettyPrint for Vec<T> {
    fn fmt_with_indent(
        &self,
        f: &mut Formatter<'_>,
        indent: usize,
    ) -> fmt::Result {
        for item in self {
            item.fmt_with_indent(f, indent)?;
        }

        Ok(())
    }
}
