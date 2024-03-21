use bonk_plaid::cli;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    cli::run(args)
}
