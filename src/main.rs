mod grid;
mod interpreter;
mod parsers;
mod stack; // Lets the compiler know about the `stack` module

use std::fs::File;

use anyhow::{anyhow, Result};
use clap::Parser;
use parsers::GratieParse;
use interpreter::Interpreter;
use stack::Stack; // Brings the `Stack` struct into scope, so you can use `Stack` directly without needing to prefix it with `stack::`.

#[derive(Parser, Debug)]
struct Args {
    /// Path to a simple text file containing a grid of colors.
    program_file: String,
}

fn main() -> Result<()> {

    let args = Args::parse();
    let f = File::open(args.program_file).expect("could not open input program file");

    // TODO(jph): check file extension to determine parse type; for now, just create a text parser
    let parser = parsers::SimpleText::default();
    let grid = parser.parse(f)?;

    Ok(())
}
