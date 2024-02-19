use anyhow::Result;
use clap::Parser;

use bonk_lsp::cli;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    cli::run(args)
}
