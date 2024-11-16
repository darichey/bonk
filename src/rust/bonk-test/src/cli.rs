use std::path::PathBuf;

use clap::Parser;

use crate::run_tests;

/// Runs snapshot tests.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long, default_value = "./Bonk.toml")]
    pub cfg: PathBuf,

    /// Whether to update snapshots on mismatch.
    #[arg(short, long)]
    pub update: bool,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let Args { cfg, update } = args;
    run_tests(cfg, update)?;
    Ok(())
}
