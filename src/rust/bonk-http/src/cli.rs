use std::{
    error::Error,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bonk_check::WorkspaceExt as _;
use bonk_db::Db;
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use clap::Parser;
use rouille::{router, Response, Server};

use crate::{get_transactions, State};

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long)]
    pub cfg: PathBuf,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let state = {
        let Args { cfg } = args;

        let workspace = Workspace::from_cfg(cfg).expect("Couldn't read cfg");
        let workspace = workspace.parse().unwrap();
        let workspace = workspace.check().unwrap();

        let state = State {
            db: Db::new(&workspace, ":memory:").expect("Couldn't create database"),
        };

        Arc::new(Mutex::new(state))
    };

    let server = Server::new("localhost:8080", move |request| {
        let state = state.clone();
        router!(request,
            (GET) (/transactions) => { get_transactions(request, state) },
            _ => Response::empty_404(),
        )
    })
    .unwrap();

    println!("Listening on port {}", server.server_addr().port());

    server.run();

    Ok(())
}
