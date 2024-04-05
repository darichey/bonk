use axum::{debug_handler, extract::State, Json};
use bonk_db::QueryOutput;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::{AppJson, AppState, BonkHttpResult};

#[derive(Deserialize)]
pub struct QueryRequest {
    query: String,
}

pub struct TableData(pub QueryOutput);

impl Serialize for TableData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut table_data = serializer.serialize_struct("TableData", 2)?;
        table_data.serialize_field("columnNames", &self.0.column_names)?;
        table_data.serialize_field("data", &self.0.data)?;
        table_data.end()
    }
}

#[debug_handler(state = AppState)]
pub async fn query_transactions(
    State(state): State<AppState>,
    Json(body): Json<QueryRequest>,
) -> BonkHttpResult<TableData> {
    let db = &state
        .mutable
        .lock()
        .expect("mutable state lock poisoned")
        .db;

    let output = db.query(&body.query)?;
    let table_data = TableData(output);

    Ok(AppJson(table_data))
}
