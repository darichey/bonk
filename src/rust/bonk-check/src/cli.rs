use anyhow::{anyhow, Result};
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use clap::Parser;
use std::path::PathBuf;

use crate::WorkspaceExt as _;

/// Checks a Bonk workspace for correctness.
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long)]
    pub cfg: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
    let Args { cfg } = args;

    let workspace = Workspace::from_cfg(cfg).map_err(|err| anyhow!(err))?;
    let workspace = workspace.parse()?;
    let _workspace = workspace.check().map_err(|err| {
        // TODO: pretty print errors with miette or something like it
        anyhow!(err
            .into_iter()
            .map(|err| format!("{:?}", err))
            .collect::<Vec<_>>()
            .join("\n"))
    })?;

    Ok(())
}
