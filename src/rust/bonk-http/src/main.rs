use anyhow::Result;
use bonk_http::cli;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    cli::run(args)
}
