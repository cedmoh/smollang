use std::fmt::{self, Display, Formatter, Write};

// Writes indentation spaces for the given indent level.
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

// Writes a label for a node, which is just the node type name.
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
