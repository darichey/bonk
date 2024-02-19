use anyhow::Result;
use bonk_plaid::cli;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    cli::run(args)
}
