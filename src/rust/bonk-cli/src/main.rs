use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Csv(bonk_csv::cli::Args),
    Db(bonk_db::cli::Args),
    Http(bonk_http::cli::Args),
    Lsp(bonk_lsp::cli::Args),
    Plaid(bonk_plaid::cli::Args),
}

fn main() -> Result<()> {
    let Args { command } = Args::parse();

    match command {
        Commands::Csv(args) => bonk_csv::cli::run(args)?,
        Commands::Db(args) => bonk_db::cli::run(args)?,
        Commands::Http(args) => bonk_http::cli::run(args)?,
        Commands::Lsp(args) => bonk_lsp::cli::run(args)?,
        Commands::Plaid(args) => bonk_plaid::cli::run(args)?,
    };

    Ok(())
}
