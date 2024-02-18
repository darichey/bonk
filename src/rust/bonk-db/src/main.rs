use bonk_db::create_db;
use clap::Parser;
use std::path::PathBuf;

/// Produces an sqlite database from a Bonk workspace.
#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long)]
    cfg: PathBuf,

    /// Path to the sqlite database.
    #[arg(short, long)]
    database: PathBuf,
}

fn main() {
    let Args { cfg, database } = Args::parse();
    create_db(cfg, database).unwrap();
}
