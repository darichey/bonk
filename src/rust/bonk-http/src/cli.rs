use std::path::PathBuf;

use clap::Parser;

/// Starts an http server that can be used to interact with the Bonk workspace
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long, default_value = "./Bonk.toml")]
    pub cfg: PathBuf,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    crate::run(&args.cfg)
}
