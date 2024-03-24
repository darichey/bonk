use axum::{debug_handler, extract::State};

use crate::{AppJson, AppState, BonkHttpResult};

#[debug_handler(state = AppState)]
pub async fn get_dashboard_names(State(state): State<AppState>) -> BonkHttpResult<Vec<String>> {
    let dashboard_names = state
        .dashboards
        .iter()
        .map(|dashboard| dashboard.name.clone())
        .collect();

    Ok(AppJson(dashboard_names))
}
