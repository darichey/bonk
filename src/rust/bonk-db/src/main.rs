use std::{fs, path::PathBuf};

use bonk_db::Db;
use clap::Parser;

/// Produces an sqlite database from a Bonk ledger.
#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Path to the Bonk ledger.
    #[arg(short, long)]
    ledger: PathBuf,

    /// Path to the sqlite database.
    #[arg(short, long)]
    database: PathBuf,
}

fn main() {
    let Args { ledger, database } = Args::parse();

    let ledger = fs::read_to_string(ledger).expect("Couldn't read ledger");
    let ledger = bonk_ast::Parser::new().parse(&ledger, None);
    let ledger = bonk_check::check_syntax(ledger).unwrap();

    Db::new(&ledger, database).expect("Couldn't create database");
}
