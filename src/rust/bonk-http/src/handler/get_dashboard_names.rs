use axum::extract::State;

use crate::{AppJson, BonkHttpResult, BonkHttpState};

pub async fn get_dashboard_names(State(state): BonkHttpState) -> BonkHttpResult<Vec<String>> {
    let dashboard_names = state
        .dashboards
        .iter()
        .map(|dashboard| dashboard.name.clone())
        .collect();

    Ok(AppJson(dashboard_names))
}
