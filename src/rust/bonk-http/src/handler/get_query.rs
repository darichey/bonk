use anyhow::Context;
use axum::{debug_handler, extract::State, Json};
use bonk_cfg::query::Query;
use serde::Deserialize;

use crate::{AppJson, AppState, BonkHttpResult};

#[derive(Deserialize)]
pub struct GetQueryRequest {
    name: String,
}

// TODO: it's weird that this is a POST with a name in the body. We should just give dashboards IDs and use that as a route param
#[debug_handler(state = AppState)]
pub async fn get_query(
    State(state): State<AppState>,
    Json(body): Json<GetQueryRequest>,
) -> BonkHttpResult<Query> {
    let queries = &state
        .mutable
        .lock()
        .expect("mutable state lock poisoned")
        .workspace
        .cfg
        .queries;

    let query = queries
        .iter()
        .find(|query| query.name == body.name)
        .cloned()
        .context("couldn't find dashboard")?;

    Ok(AppJson(query))
}
