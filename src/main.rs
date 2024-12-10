mod grid;
mod interpreter;
mod parsers;
mod stack; // Lets the compiler know about the `stack` module

use std::fs::File;

use anyhow::{anyhow, Result};
use clap::Parser;
use easy_repl::{command, CommandStatus, Repl};
use interpreter::Interpreter;
use parsers::GratieParse;
use stack::Stack; // Brings the `Stack` struct into scope, so you can use `Stack` directly without needing to prefix it with `stack::`.

#[derive(Parser, Debug)]
struct Args {
    /// Path to a simple text file containing a grid of colors.
    program_file: String,

    /// Run the interpreter in debugger mode
    #[arg(short, default_value = "false")]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(args.program_file).expect("could not open input program file");

    // TODO(jph): check file extension to determine parse type; for now, just create a text parser
    let parser = parsers::SimpleText::default();
    let grid = parser.parse(f)?;

    let interpreter = Interpreter::new(grid.clone());
    let g = grid.clone();

    if args.debug {
        let mut repl = Repl::builder()
            .add(
                "run",
                command! {
                    "run loaded program",
                    () => || {
                        interpreter.run();

                        Ok(CommandStatus::Done)
                    }
                },
            )
            .add(
                "color-block",
                command! {
                    "find all codels in a color block for a given index",
                    (row: usize, col: usize) => | row, col | {
                        let res = g.find_codel_block((row, col));
                        println!("{:?}", res);

                        Ok(CommandStatus::Done)
                    }
                },
            )
            .add(
                "grid",
                command! {
                    "print the grid of the current program",
                    () => || {
                        println!("{:?}", grid.clone());
                        Ok(CommandStatus::Done)
                    }
                },
            )
            .build()
            .expect("could not create repl");
        repl.run().expect("Critical REPL error");
    } else {
        interpreter.run();
    }

    Ok(())
}
