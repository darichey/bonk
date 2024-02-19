use bonk_plaid::cli;
use clap::Parser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    cli::run(args)
}
