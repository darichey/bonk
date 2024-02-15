use std::{fs, path::PathBuf};

use bonk_check::CheckUnit;
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
    let Args {
        ledger: ledger_path,
        database,
    } = Args::parse();

    let src = fs::read_to_string(&ledger_path).expect("Couldn't read ledger");
    let ledger = bonk_ast::Parser::new().parse(&src, None);
    let check_unit = CheckUnit::new(vec![(ledger_path.clone(), &ledger)]);
    let check_unit = check_unit
        .check(&CheckUnit::new(vec![(ledger_path, &src)]))
        .unwrap();

    Db::new(&check_unit, database).expect("Couldn't create database");
}
