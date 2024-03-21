use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, Mutex},
};

use rouille::{Request, Response};
use serde::Serialize;

use crate::{try_or_400, State};

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

pub(crate) fn get_transactions(request: &Request, state: Arc<Mutex<State>>) -> Response {
    let body = try_or_400!(handle(request, state));
    Response::json(&body)
}

const QUERY: &str = r#"SELECT id,date,description,account,amount FROM "transaction" INNER JOIN posting ON "transaction".id = posting."transaction""#;

// TODO: order by date
// TODO: paginate by date
fn handle(_request: &Request, state: Arc<Mutex<State>>) -> anyhow::Result<Vec<Transaction>> {
    let state = state.lock().expect("Couldn't acquire state");
    let con = &state.db.con;

    let mut transactions: HashMap<i64, Transaction> = HashMap::new();

    for row in con.prepare(QUERY)?.into_iter() {
        let row = row?;
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

    Ok(transactions.into_values().collect::<Vec<_>>())
}
