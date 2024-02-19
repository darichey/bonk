use anyhow::Result;
use bonk_csv::cli;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    cli::run(args)
}
