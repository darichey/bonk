use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{AppJson, BonkHttpResult, BonkHttpState, SqlValue};

#[derive(Deserialize)]
pub struct QueryRequest {
    query: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableData {
    column_names: Vec<String>,
    data: Vec<Vec<SqlValue>>,
}

pub async fn query_transactions(
    State(state): BonkHttpState,
    Json(body): Json<QueryRequest>,
) -> BonkHttpResult<TableData> {
    let con = &state.db.lock().expect("db lock poisoned").con;

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

    let table_data = TableData { column_names, data };

    Ok(AppJson(table_data))
}
