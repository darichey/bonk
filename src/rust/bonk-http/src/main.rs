use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bonk_check::WorkspaceExt as _;
use bonk_db::Db;
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use clap::Parser;
use rouille::{router, Request, Response, Server};
use serde::Serialize;

struct State {
    db: Db,
}

impl State {}

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Path to the Bonk workspace config.
    #[arg(short, long)]
    cfg: PathBuf,
}

#[derive(Serialize)]
struct Transaction {
    date: String,
    description: String,
    postings: Vec<Posting>,
}

#[derive(Serialize)]
struct Posting {
    account: String,
    amount: i32,
}

// TODO: order by date
// TODO: paginate by date
fn get_transactions(_request: &Request, state: Arc<Mutex<State>>) -> Response {
    let state = state.lock().expect("Couldn't acquire state");
    let con = &state.db.con;

    let mut transactions: HashMap<i64, Transaction> = HashMap::new();

    for row in con.prepare(r#"SELECT id,date,description,account,amount FROM "transaction" INNER JOIN posting ON "transaction".id = posting."transaction""#).unwrap().into_iter() {
        let row = row.unwrap();
        let id = row.read::<i64, _>("id");
        let date = row.read::<&str, _>("date");
        let description = row.read::<&str, _>("description");
        let account = row.read::<&str, _>("account");
        let amount = row.read::<i64, _>("amount") as i32;

        let posting = Posting {
            account: account.to_string(),
            amount,
        };

        match transactions.entry(id) {
            Entry::Occupied(mut entry) => entry.get_mut().postings.push(posting),
            Entry::Vacant(entry) => {
                entry.insert(Transaction {
                    date: date.to_string(),
                    description: description.to_string(),
                    postings: vec![posting],
                });
            }
        }
    }

    Response::json(&transactions.values().collect::<Vec<_>>())
        .with_additional_header("Access-Control-Allow-Origin", "*") // TODO real cors
}

fn main() {
    let state = {
        let Args { cfg } = Args::parse();

        let workspace = Workspace::from_cfg(&cfg).expect("Couldn't read cfg");
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

    server.run()
}
