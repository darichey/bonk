use std::sync::{Arc, Mutex};

use rouille::{Request, Response};
use serde::{Deserialize, Serialize};

use crate::{try_or_400, SqlValue, State};

#[derive(Deserialize)]
struct QueryRequest {
    query: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TableData {
    column_names: Vec<String>,
    data: Vec<Vec<SqlValue>>,
}

pub(crate) fn query_transactions(request: &Request, state: Arc<Mutex<State>>) -> Response {
    let body = try_or_400!(handle(request, state));
    Response::json(&body)
}

fn handle(request: &Request, state: Arc<Mutex<State>>) -> anyhow::Result<TableData> {
    let state = state.lock().expect("Couldn't acquire state");
    let con = &state.db.con;

    let body: QueryRequest = rouille::input::json_input(request)?;

    let stmt = con.prepare(body.query)?;

    let column_names = stmt.column_names().to_vec();

    let data = stmt
        .into_iter()
        .map(|row| {
            let values: Vec<sqlite::Value> = row?.into();
            let values: Vec<SqlValue> = values.into_iter().map(SqlValue).collect();
            Ok(values)
        })
        .collect::<anyhow::Result<Vec<Vec<SqlValue>>>>()?;

    Ok(TableData { column_names, data })
}
