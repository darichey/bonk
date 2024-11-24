use clap::Parser;

use std::path::PathBuf;

use crate::run_server;

/// Starts a Bonk LSP server
#[derive(Parser)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long, default_value = "./Bonk.toml")]
    pub cfg: PathBuf,

    /// vscode-languageserver-node passes --stdio when starting the server.
    #[arg(long, hide = true)]
    pub stdio: bool,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let Args { cfg, .. } = args;
    run_server(cfg)?;
    Ok(())
}
