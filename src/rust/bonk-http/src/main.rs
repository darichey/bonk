use std::error::Error;

use bonk_http::cli;
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::parse();
    cli::run(args)
}
