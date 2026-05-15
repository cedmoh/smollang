use std::fmt::{self, Display, Formatter, Write};

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

/// Writes a single indentation level using two spaces per level.
pub fn write_indent(f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
    for _ in 0..indent {
        f.write_char(' ')?;
    }

    Ok(())
}

/// Writes a `<none>` placeholder for an optional field that is `None`.
pub fn write_none(f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
    write_indent(f, indent)?;
    writeln!(f, "<none>")
}

/// Writes an `<empty>` placeholder for an empty list or block of code.
pub fn write_empty(f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
    write_indent(f, indent)?;
    writeln!(f, "<empty>")
}

/// Writes a node label prefixed with `-`.
pub fn write_node_label(
    f: &mut Formatter<'_>,
    indent: usize,
    label: &str,
) -> fmt::Result {
    write_indent(f, indent)?;
    writeln!(f, "{label}")
}

/// Writes a scalar field in `name: value` form.
pub fn write_scalar_field<T: Display>(
    f: &mut Formatter<'_>,
    indent: usize,
    name: &str,
    value: T,
) -> fmt::Result {
    write_indent(f, indent)?;
    writeln!(f, "{name}: {value}")
}

/// Writes a field label that will be followed by a nested value.
pub fn write_field_label(
    f: &mut Formatter<'_>,
    indent: usize,
    name: &str,
) -> fmt::Result {
    write_indent(f, indent)?;
    writeln!(f, "{name}:")
}
