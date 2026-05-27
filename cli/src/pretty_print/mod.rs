mod ast_implementations;
mod helpers;

pub use helpers::*;

use std::fmt::{self, Display, Formatter};

/// The default indentation string used when nesting child nodes.
pub(crate) static INDENT: &str = "  ";

/// Formats AST nodes using a tree representation with box-drawing symbols.
pub trait PrettyPrint {
    /// Formats this value, prepending `prefix` to every output line.
    fn fmt_with_indent(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result;

    /// Returns a wrapper that renders this value using [`Display`].
    fn pretty(&self, colors_enabled: bool) -> PrettyPrinter<'_, Self>
    where
        Self: Sized,
    {
        PrettyPrinter {
            value: self,
            colors_enabled,
        }
    }
}

/// A [`Display`] wrapper for any [`PrettyPrint`] implementation.
pub struct PrettyPrinter<'a, T: ?Sized> {
    /// The value being wrapped.
    value: &'a T,
    /// Whether ANSI colors are enabled.
    colors_enabled: bool,
}

impl<'a, T: PrettyPrint + ?Sized> Display for PrettyPrinter<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.value.fmt_with_indent(f, "", self.colors_enabled)
    }
}

impl<T: PrettyPrint> PrettyPrint for Vec<T> {
    fn fmt_with_indent(
        &self,
        f: &mut Formatter<'_>,
        prefix: &str,
        colors_enabled: bool,
    ) -> fmt::Result {
        for item in self {
            item.fmt_with_indent(f, prefix, colors_enabled)?;
        }

        Ok(())
    }
}
