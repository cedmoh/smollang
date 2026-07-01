mod pretty_print;
mod standard_io;
mod styles;

use clap::{Arg, Command, command};
use compiler::Compiler;
use pretty_print::PrettyPrint;
use std::path::PathBuf;
use styles::CARGO_STYLING;
use vm::Vm;

use crate::{
    pretty_print::{AssemblyPrettyPrint, MemoryPrettyPrint},
    standard_io::StandardIo,
};

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
        .arg(
            Arg::new("dump")
                .help("Dump the VM memory after execution.")
                .long("dump")
                .action(clap::ArgAction::SetTrue)
                .requires("file"),
        )
        .subcommand(
            Command::new("compile")
                .alias("c")
                .about("Compile the file at the given path and print the resulting assembly.")
                .arg(
                    Arg::new("file")
                        .help("Path to file to compile.")
                        .value_parser(clap::value_parser!(PathBuf))
                        .required(true),
                )
        )
        .subcommand(
            Command::new("parse")
                .alias("p")
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
            Command::new("format")
                .alias("f")
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
                let dump: bool =
                    matches.get_one("dump").copied().unwrap_or(false);
                execute_file(file, dump)
            } else {
                repl()
            }
        }
        // Provided Subcommand is `format`
        Some(("format", sub_m)) => {
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
        // Provided Subcommand is `compile`
        Some(("compile", sub_m)) => {
            let file: &PathBuf = sub_m.get_one("file").unwrap();
            compile_file(file)
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

fn compile_file(path: &PathBuf) -> anyhow::Result<()> {
    let input = std::fs::read_to_string(path)?;

    let ast = parser::parse_string_to_program_ast(&input)?;

    let mut compiler = Compiler::new();
    let assembly = compiler.compile(ast)?.chunk;

    println!("{}", assembly.pretty_print(true));

    Ok(())
}

fn execute_file(path: &PathBuf, dump: bool) -> anyhow::Result<()> {
    let input = std::fs::read_to_string(path)?;

    let ast = parser::parse_string_to_program_ast(&input)?;

    let mut compiler = Compiler::new();
    let assembly = compiler.compile(ast)?.chunk;

    let io = StandardIo::new();

    let mut vm = Vm::new_with_io(io);

    match vm.load_assembly(assembly).run() {
        x => {
            if dump {
                println!("=== Stack === \n{}", vm.stack.pretty_print(true));
                println!("=== Memory === \n{}", vm.memory.pretty_print(true));
            }

            x?
        }
    }

    Ok(())
}

fn format_path(path: &PathBuf) -> anyhow::Result<()> {
    println!("Formatting files at path: {:?}", path);
    todo!()
}

fn repl() -> anyhow::Result<()> {
    todo!()
}
