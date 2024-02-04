use std::{
    error::Error,
    fs::{self},
    path::PathBuf,
};

use bonk_csv::do_convert;
use clap::Parser;

/// Produces a partial Bonk ledger by converting transactions from a csv file with the header `date,description,amount`.
#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// The Bonk account to associate the converted transactions to (e.g., "assets:my_checking").
    #[arg(short, long)]
    account: String,

    /// The path to the csv file to read from (e.g., "./foo.csv").
    #[arg(short, long)]
    input: PathBuf,

    /// The path to output the ledger to (e.g., "./foo.partial.bonk").
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        account,
        input,
        output,
    } = Args::parse();

    let mut reader = csv::Reader::from_path(input)?;
    let ledger = do_convert(&account, &mut reader)?;
    fs::write(output, ledger.to_string())?;

    Ok(())
}
