use std::fmt::{self, Display, Formatter};

use colored::{ColoredString, Colorize};

/// Returns the number of visible (non-ANSI) characters in `s`.
///
/// Each level in the prefix is always exactly 4 display characters wide
/// (`│   ` or `    `), so `display_len(prefix) / 4` gives the nesting depth.
fn display_len(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip the entire ANSI escape sequence: \x1b[ ... m
            for c in chars.by_ref() {
                if c == 'm' {
                    break;
                }
            }
        } else {
            count += 1;
        }
    }
    count
}

/// Applies one of 8 cycling level colors to a string.
fn color_for_level(s: &str, level: usize) -> ColoredString {
    match level % 8 {
        0 => s.cyan(),
        1 => s.green(),
        2 => s.yellow(),
        3 => s.magenta(),
        4 => s.blue(),
        5 => s.red(),
        6 => s.bright_cyan(),
        7 => s.bright_yellow(),
        _ => unreachable!(),
    }
}

/// Writes a node label (the node type name) at the current prefix.
pub fn write_node_label(
    f: &mut Formatter<'_>,
    prefix: &str,
    colors_enabled: bool,
    label: &str,
) -> fmt::Result {
    if colors_enabled {
        writeln!(f, "{}{}", prefix, label.yellow().bold())
    } else {
        writeln!(f, "{prefix}{label}")
    }
}

/// Writes a `<none>` placeholder for an optional field that is `None`.
pub fn write_none(
    f: &mut Formatter<'_>,
    prefix: &str,
    colors_enabled: bool,
) -> fmt::Result {
    if colors_enabled {
        writeln!(f, "{}{}", prefix, "<none>".bright_black().italic())
    } else {
        writeln!(f, "{prefix}<none>")
    }
}

/// Writes an `<empty>` placeholder for an empty list or block of code.
pub fn write_empty(
    f: &mut Formatter<'_>,
    prefix: &str,
    colors_enabled: bool,
) -> fmt::Result {
    if colors_enabled {
        writeln!(f, "{}{}", prefix, "<empty>".bright_black().italic())
    } else {
        writeln!(f, "{prefix}<empty>")
    }
}

/// Writes a tree branch connector for a named field, followed by a colon.
///
/// Outputs `├── name:` for a non-last field and `└── name:` for the last field.
/// When colors are enabled the connector characters are tinted with the color
/// assigned to the current nesting level.
pub fn write_tree_field_label(
    f: &mut Formatter<'_>,
    prefix: &str,
    colors_enabled: bool,
    name: &str,
    is_last: bool,
) -> fmt::Result {
    let connector = if is_last { "└── " } else { "├── " };

    if colors_enabled {
        let level = display_len(prefix) / 4;
        writeln!(
            f,
            "{}{}{}:",
            prefix,
            color_for_level(connector, level),
            name.red()
        )
    } else {
        writeln!(f, "{prefix}{connector}{name}:")
    }
}

/// Returns the child-level prefix to use after a tree branch connector.
///
/// Uses `│   ` for non-last branches so the vertical line continues, and
/// four spaces for the last branch. When `colors_enabled` is `true` the `│`
/// character is tinted with the color assigned to the current nesting level.
pub fn tree_child_prefix(
    prefix: &str,
    is_last: bool,
    colors_enabled: bool,
) -> String {
    if is_last {
        format!("{prefix}    ")
    } else if colors_enabled {
        let level = display_len(prefix) / 4;
        format!("{prefix}{}   ", color_for_level("│", level))
    } else {
        format!("{prefix}│   ")
    }
}

/// Writes a scalar leaf field with a tree branch connector in `├──/└── name:
/// value` form. When colors are enabled the connector is tinted with the color
/// assigned to the current nesting level.
pub fn write_tree_scalar_field<T: Display>(
    f: &mut Formatter<'_>,
    prefix: &str,
    colors_enabled: bool,
    name: &str,
    value: T,
    is_last: bool,
) -> fmt::Result {
    let connector = if is_last { "└── " } else { "├── " };

    if colors_enabled {
        let level = display_len(prefix) / 4;
        writeln!(
            f,
            "{}{}{}: {}",
            prefix,
            color_for_level(connector, level),
            name.red(),
            value.to_string().green()
        )
    } else {
        writeln!(f, "{prefix}{connector}{name}: {value}")
    }
}
