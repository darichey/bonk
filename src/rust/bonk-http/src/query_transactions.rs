use std::sync::{Arc, Mutex};

use rouille::{try_or_400, Request, Response};
use serde::{Deserialize, Serialize};

use crate::{SqlValue, State};

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

// TODO: real error handling
pub(crate) fn query_transactions(request: &Request, state: Arc<Mutex<State>>) -> Response {
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
            Response::json(&response)
        }
        Err(err) => Response::json(&err.to_string()).with_status_code(400),
    }
}
