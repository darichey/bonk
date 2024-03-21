use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, Mutex},
};

use rouille::{Request, Response};
use serde::Serialize;

use crate::State;

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
pub(crate) fn get_transactions(_request: &Request, state: Arc<Mutex<State>>) -> Response {
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
