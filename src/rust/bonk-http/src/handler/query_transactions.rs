use axum::{debug_handler, extract::State, Json};
use bonk_db::SqlValue;
use serde::{Deserialize, Serialize};

use crate::{AppJson, AppState, BonkHttpResult};

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

#[debug_handler(state = AppState)]
pub async fn query_transactions(
    State(state): State<AppState>,
    Json(body): Json<QueryRequest>,
) -> BonkHttpResult<TableData> {
    let con = &state
        .mutable
        .lock()
        .expect("mutable state lock poisoned")
        .db
        .con;

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
