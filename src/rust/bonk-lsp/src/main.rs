use clap::Parser;

use bonk_lsp::cli;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    cli::run(args)
}
