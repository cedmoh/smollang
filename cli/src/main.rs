mod pretty_print;
mod styles;

use clap::{Arg, Command, command};
use pretty_print::PrettyPrint;
use std::path::PathBuf;
use styles::CARGO_STYLING;

fn main() -> anyhow::Result<()> {
    let command = command!()
        .styles(CARGO_STYLING)
        .about("A CLI tool for working with projects written in smollang")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .help("Run the script pointed to by the given path.")
                .value_parser(clap::value_parser!(PathBuf))
                .required(false)
                .index(1),
        )
        .subcommand(
            Command::new("parse")
                .about("Parse the file at the given path and print the resulting AST.")
                .arg(
                    Arg::new("file")
                        .help("Path to file to parse.")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                )
                .arg(
                    Arg::new("debug")
                        .help("Print the resulting AST using debug formatting.")
                        .long("debug")
                        .short('d')
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("no-color")
                        .help("Disable colored pretty-printed AST output.")
                        .long("no-color")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("fmt")
                .about("Format files in the given path recursively.")
                .arg(
                    Arg::new("path")
                        .help("Path to format recursively.")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                ),
        );

    let matches = command.get_matches();

    match matches.subcommand() {
        // No subcommand provided.
        None => {
            if let Some(file) = matches.get_one::<PathBuf>("file") {
                execute_file(file)
            } else {
                repl()
            }
        }
        // Provided Subcommand is `fmt`
        Some(("fmt", sub_m)) => {
            let path: &PathBuf = sub_m.get_one("path").unwrap();
            format_path(path)
        }
        // Provided Subcommand is `parse`
        Some(("parse", sub_m)) => {
            let file: &PathBuf = sub_m.get_one("file").unwrap();
            let is_debug: bool =
                sub_m.get_one("debug").copied().unwrap_or(false);
            let no_color: bool =
                sub_m.get_one("no-color").copied().unwrap_or(false);

            parse_file(file, is_debug, no_color)
        }
        _ => {
            unreachable!(
                "Any unhandled argument was assumed to be handled as path to file."
            )
        }
    }
}

fn parse_file(
    path: &PathBuf,
    is_debug: bool,
    no_color: bool,
) -> anyhow::Result<()> {
    let input = std::fs::read_to_string(path)?;

    let ast = parser::parse_string_to_program_ast(&input)?;

    if is_debug {
        println!("{:#?}", ast);
    } else {
        println!("{}", ast.pretty(!no_color));
    }

    Ok(())
}

fn execute_file(path: &PathBuf) -> anyhow::Result<()> {
    println!("Executing file at path: {:?}", path);
    todo!()
}

fn format_path(path: &PathBuf) -> anyhow::Result<()> {
    println!("Formatting files at path: {:?}", path);
    todo!()
}

fn repl() -> anyhow::Result<()> {
    todo!()
}
