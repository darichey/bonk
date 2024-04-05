use std::collections::HashMap;

use axum::{debug_handler, extract::State, Json};
use bonk_db::{QueryOutput, SqlValue};
use serde::Deserialize;

use crate::{AppJson, AppState, BonkHttpResult};

#[derive(Deserialize)]
pub struct QueryRequest {
    query: String,
}

#[debug_handler(state = AppState)]
pub async fn query_transactions_for_chart(
    State(state): State<AppState>,
    Json(body): Json<QueryRequest>,
) -> BonkHttpResult<HashMap<String, Vec<SqlValue>>> {
    let db = &state
        .mutable
        .lock()
        .expect("mutable state lock poisoned")
        .db;

    let QueryOutput { column_names, data } = db.query(&body.query)?;

    let mut data_iters: Vec<_> = data.into_iter().map(Vec::into_iter).collect();

    let columns: Vec<Vec<SqlValue>> = (0..column_names.len())
        .map(|_| data_iters.iter_mut().map(|it| it.next().unwrap()).collect())
        .collect();

    let query_result = column_names.into_iter().zip(columns).collect();

    Ok(AppJson(query_result))
}
