use clap::Parser;

use std::path::PathBuf;

use crate::run_server;

/// Starts a Bonk LSP server
#[derive(Parser)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    pub cfg_path: PathBuf,

    #[arg(long)]
    pub stdio: bool,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let Args { cfg_path, .. } = args;
    run_server(cfg_path)?;
    Ok(())
}
