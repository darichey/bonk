use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use bonk_check::WorkspaceExt as _;
use bonk_db::Db;
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use clap::Parser;
use rouille::{router, Response, Server};

use crate::{get_transactions, query_transactions, State};

/// Starts an http server that can be used to interact with the Bonk workspace
#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long)]
    pub cfg: PathBuf,
}

pub fn run(args: Args) -> Result<()> {
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
        if request.method() == "OPTIONS" {
            // TOOD: real cors
            return Response::empty_204()
                .with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Methods", "*")
                .with_additional_header("Access-Control-Allow-Headers", "*");
        }

        let response = router!(request,
            (GET) (/transactions) => { get_transactions(request, state.clone()) },
            (POST) (/queryTransactions) => { query_transactions(request, state.clone()) },
            _ => Response::empty_404(),
        );

        // TODO: real cors
        response.with_additional_header("Access-Control-Allow-Origin", "*")
    })
    .unwrap();

    println!("Listening on port {}", server.server_addr().port());

    server.run();

    Ok(())
}
