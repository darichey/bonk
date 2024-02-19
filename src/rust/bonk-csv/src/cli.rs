use std::{error::Error, fs, path::PathBuf};

use clap::Parser;

use crate::do_convert;

/// Produces a partial Bonk ledger by converting transactions from a csv file with the header `date,description,amount`.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// The Bonk account to associate the converted transactions to (e.g., "assets:my_checking").
    #[arg(short, long)]
    pub account: String,

    /// The path to the csv file to read from (e.g., "./foo.csv").
    #[arg(short, long)]
    pub input: PathBuf,

    /// The path to output the ledger to (e.g., "./foo.partial.bonk").
    #[arg(short, long)]
    pub output: PathBuf,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let Args {
        account,
        input,
        output,
    } = args;

    let mut reader = csv::Reader::from_path(input)?;
    let ledger = do_convert(&account, &mut reader)?;
    fs::write(output, ledger.to_string())?;

    Ok(())
}
