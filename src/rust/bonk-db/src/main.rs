use bonk_check::WorkspaceExt;
use bonk_db::Db;
use bonk_workspace::Workspace;
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

    let workspace = Workspace::from_cfg(&cfg).expect("Couldn't read cfg");
    let workspace = workspace.check().unwrap();

    Db::new(&workspace, database).expect("Couldn't create database");
}
