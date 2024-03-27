use axum::{debug_handler, extract::State};

use crate::{AppJson, AppState, BonkHttpResult};

#[debug_handler(state = AppState)]
pub async fn get_query_names(State(state): State<AppState>) -> BonkHttpResult<Vec<String>> {
    let queries = &state
        .mutable
        .lock()
        .expect("mutable state lock poisoned")
        .workspace
        .cfg
        .queries;

    let query_names = queries.iter().map(|query| query.name.clone()).collect();

    Ok(AppJson(query_names))
}
