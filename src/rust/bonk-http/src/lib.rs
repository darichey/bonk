pub mod cli;

use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, Mutex},
};

use bonk_db::Db;

use rouille::{try_or_400, Request, Response};
use serde::{Deserialize, Serialize};

struct State {
    db: Db,
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

#[derive(Deserialize)]
struct QueryRequest {
    query: String,
}

/// A wrapper for seralizing sqlite Values
pub struct SqlValue(sqlite::Value);

impl Serialize for SqlValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            sqlite::Value::Binary(b) => serializer.serialize_bytes(b),
            sqlite::Value::Float(f) => serializer.serialize_f64(*f),
            sqlite::Value::Integer(i) => serializer.serialize_i64(*i),
            sqlite::Value::String(s) => serializer.serialize_str(s),
            sqlite::Value::Null => serializer.serialize_unit(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TableData {
    column_names: Vec<String>,
    data: Vec<Vec<SqlValue>>,
}

// TODO: real error handling
fn query_transactions(request: &Request, state: Arc<Mutex<State>>) -> Response {
    let state = state.lock().expect("Couldn't acquire state");
    let con = &state.db.con;

    let body: QueryRequest = try_or_400!(rouille::input::json_input(request));

    let stmt = try_or_400!(con.prepare(body.query));

    let column_names = stmt.column_names().to_vec();

    let data = stmt
        .into_iter()
        .map(|row| {
            let values: Vec<sqlite::Value> = row?.into();
            let values: Vec<SqlValue> = values.into_iter().map(SqlValue).collect();
            Ok(values)
        })
        .collect::<anyhow::Result<Vec<Vec<SqlValue>>>>();

    match data {
        Ok(data) => {
            let response = TableData { column_names, data };

            // TODO real cors
            Response::json(&response).with_additional_header("Access-Control-Allow-Origin", "*")
        }
        Err(err) => Response::json(&err.to_string()).with_status_code(400),
    }
}
