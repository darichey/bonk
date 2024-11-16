use clap::Parser;
use std::path::PathBuf;

use crate::exec_query;

/// Executes an SQLite query against the database produced from a Bonk workspace.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long, default_value = "./Bonk.toml")]
    pub cfg: PathBuf,

    pub query: String,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let Args { cfg, query } = args;
    exec_query(cfg, &query)?;
    Ok(())
}
