use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bonk_db::Db;
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
    /// Path to the Bonk ledger.
    #[arg(short, long)]
    ledger: PathBuf,
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
}

fn main() {
    let state = {
        let Args { ledger } = Args::parse();

        let src = fs::read_to_string(ledger).expect("Couldn't read ledger");
        let ledger = bonk_ast::Parser::new().parse(&src, None);
        let ledger = bonk_check::check_syntax(&ledger, &src).unwrap();

        let state = State {
            db: Db::new(&ledger, ":memory:").expect("Couldn't create database"),
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
