use std::{
    fs::File,
    io::{self, stdin, stdout},
};

use anyhow::Result;
use clap::Parser;

use crate::do_convert;

/// Produces a partial Bonk ledger by converting transactions from a csv file with the header `date,description,amount`.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// The Bonk account to associate the converted transactions to (e.g., "assets/my_checking").
    #[arg(short, long)]
    pub account: String,

    /// The path to the csv file to read from (e.g., "./foo.csv"). If not given, or "-" is given, read from stdin.
    #[arg(short, long)]
    pub input: Option<String>,

    /// The path to output the ledger to (e.g., "./foo.partial.bonk"). If not given, or "-" is given, write to stdout.
    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn run(args: Args) -> Result<()> {
    let Args {
        account,
        input,
        output,
    } = args;

    let mut reader = {
        let r: Box<dyn io::Read> = match input.as_deref() {
            Some("-") | None => Box::new(stdin().lock()),
            Some(input) => Box::new(File::open(input)?),
        };
        csv::Reader::from_reader(r)
    };

    let ledger = do_convert(&account, &mut reader)?;

    let mut w: Box<dyn io::Write> = match output.as_deref() {
        Some("-") | None => Box::new(stdout().lock()),
        Some(output) => Box::new(File::open(output)?),
    };

    w.write_fmt(format_args!("{}", ledger))?;

    Ok(())
}
