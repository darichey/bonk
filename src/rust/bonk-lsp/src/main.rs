use std::error::Error;

use clap::Parser;

use bonk_lsp::cli;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = cli::Args::parse();
    cli::run(args)
}
