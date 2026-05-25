use std::fmt::{self, Display, Formatter, Write};

use colored::Colorize;

// Writes indentation spaces for the given indent level.
pub fn write_indent(f: &mut Formatter<'_>, indent: usize) -> fmt::Result {
    for _ in 0..indent {
        f.write_char(' ')?;
    }

    Ok(())
}

/// Writes a `<none>` placeholder for an optional field that is `None`.
pub fn write_none(
    f: &mut Formatter<'_>,
    indent: usize,
    colors_enabled: bool,
) -> fmt::Result {
    write_indent(f, indent)?;

    if colors_enabled {
        writeln!(f, "{}", "<none>".bright_black().italic())
    } else {
        writeln!(f, "<none>")
    }
}

/// Writes an `<empty>` placeholder for an empty list or block of code.
pub fn write_empty(
    f: &mut Formatter<'_>,
    indent: usize,
    colors_enabled: bool,
) -> fmt::Result {
    write_indent(f, indent)?;

    if colors_enabled {
        writeln!(f, "{}", "<empty>".bright_black().italic())
    } else {
        writeln!(f, "<empty>")
    }
}

// Writes a label for a node, which is just the node type name.
pub fn write_node_label(
    f: &mut Formatter<'_>,
    indent: usize,
    colors_enabled: bool,
    label: &str,
) -> fmt::Result {
    write_indent(f, indent)?;

    if colors_enabled {
        writeln!(f, "{}", label.yellow().bold())
    } else {
        writeln!(f, "{label}")
    }
}

/// Writes a scalar field in `name: value` form.
pub fn write_scalar_field<T: Display>(
    f: &mut Formatter<'_>,
    indent: usize,
    colors_enabled: bool,
    name: &str,
    value: T,
) -> fmt::Result {
    write_indent(f, indent)?;

    if colors_enabled {
        writeln!(f, "{}: {}", name.red(), value.to_string().green())
    } else {
        writeln!(f, "{name}: {value}")
    }
}

/// Writes a field label that will be followed by a nested value.
pub fn write_field_label(
    f: &mut Formatter<'_>,
    indent: usize,
    colors_enabled: bool,
    name: &str,
) -> fmt::Result {
    write_indent(f, indent)?;

    if colors_enabled {
        writeln!(f, "{}:", name.red())
    } else {
        writeln!(f, "{name}:")
    }
}
