use std::collections::HashMap;

use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{AppJson, BonkHttpResult, BonkHttpState, SqlValue};

#[derive(Deserialize)]
pub struct QueryRequest {
    query: String,
}

pub async fn query_transactions_for_chart(
    State(state): BonkHttpState,
    Json(body): Json<QueryRequest>,
) -> BonkHttpResult<HashMap<String, Vec<SqlValue>>> {
    let state = state.lock().expect("Couldn't acquire state");
    let con = &state.db.con;

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

    let query_result = column_names.into_iter().zip(columns).collect();

    Ok(AppJson(query_result))
}
