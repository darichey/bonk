use std::{
    fs::File,
    io::{self, stdin, stdout, BufReader, Read as _},
};

use anyhow::Context as _;
use clap::Parser;

use crate::sort_ledger;

/// Sorts transactions in a Bonk ledger by date.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// The path to the ledger to read from (e.g., "./foo.bonk"). If not given, or "-" is given, read from stdin.
    #[arg(short, long)]
    pub input: Option<String>,

    /// The path to output the ledger to (e.g., "./bar.bonk"). If not given, or "-" is given, write to stdout.
    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let Args { input, output } = args;

    let src = {
        let r: Box<dyn io::Read> = match input.as_deref() {
            Some("-") | None => Box::new(stdin().lock()),
            Some(input) => Box::new(
                File::open(input)
                    .with_context(|| format!("couldn't open input file: {}", input))?,
            ),
        };
        let mut src = String::new();
        BufReader::new(r).read_to_string(&mut src)?;
        src
    };

    let ledger = sort_ledger(&src);

    let mut w: Box<dyn io::Write> = match output.as_deref() {
        Some("-") | None => Box::new(stdout().lock()),
        Some(output) => Box::new(
            File::create(output)
                .with_context(|| format!("couldn't create output file: {}", output))?,
        ),
    };

    w.write_fmt(format_args!("{}", ledger))?;

    Ok(())
}
