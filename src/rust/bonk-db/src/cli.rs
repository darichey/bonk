use clap::Parser;
use std::{error::Error, path::PathBuf};

use crate::create_db;

/// Produces an sqlite database from a Bonk workspace.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long)]
    pub cfg: PathBuf,

    /// Path to the sqlite database.
    #[arg(short, long)]
    pub database: PathBuf,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let Args { cfg, database } = args;
    create_db(cfg, database)?;
    Ok(())
}
