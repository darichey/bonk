use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Check(bonk_check::cli::Args),
    Csv(bonk_csv::cli::Args),
    Db(bonk_db::cli::Args),
    Http(bonk_http::cli::Args),
    Lsp(bonk_lsp::cli::Args),
    Plaid(bonk_plaid::cli::Args),
    Test(bonk_test::cli::Args),
    Query(bonk_query::cli::Args),
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("Error: {:#?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let Args { command } = Args::parse();

    match command {
        Commands::Check(args) => bonk_check::cli::run(args)?,
        Commands::Csv(args) => bonk_csv::cli::run(args)?,
        Commands::Db(args) => bonk_db::cli::run(args)?,
        Commands::Http(args) => bonk_http::cli::run(args)?,
        Commands::Lsp(args) => bonk_lsp::cli::run(args)?,
        Commands::Plaid(args) => bonk_plaid::cli::run(args)?,
        Commands::Test(args) => bonk_test::cli::run(args)?,
        Commands::Query(args) => bonk_query::cli::run(args)?,
    };

    Ok(())
}
