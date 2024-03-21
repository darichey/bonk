use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rouille::{Request, Response};
use serde::Deserialize;

use crate::{try_or_400, SqlValue, State};

#[derive(Deserialize)]
struct QueryRequest {
    query: String,
}

pub(crate) fn query_transactions_for_chart(
    request: &Request,
    state: Arc<Mutex<State>>,
) -> Response {
    let body = try_or_400!(handle(request, state));
    Response::json(&body)
}

fn handle(
    request: &Request,
    state: Arc<Mutex<State>>,
) -> anyhow::Result<HashMap<String, Vec<SqlValue>>> {
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

    let mut data_iters: Vec<_> = data.into_iter().map(Vec::into_iter).collect();

    let columns: Vec<Vec<SqlValue>> = (0..column_names.len())
        .map(|_| data_iters.iter_mut().map(|it| it.next().unwrap()).collect())
        .collect();

    Ok(column_names.into_iter().zip(columns).collect())
}
