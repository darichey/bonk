use bonk_check::cli;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    cli::run(args)
}
