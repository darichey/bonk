use anyhow::Context;
use axum::{extract::State, Json};
use bonk_dashboard::Dashboard;
use serde::Deserialize;

use crate::{AppJson, BonkHttpResult, BonkHttpState};

#[derive(Deserialize)]
pub struct GetDashboardRequest {
    name: String,
}

// TODO: it's weird that this is a POST with a name in the body. We should just give dashboards IDs and use that as a route param
pub async fn get_dashboard(
    State(state): BonkHttpState,
    Json(body): Json<GetDashboardRequest>,
) -> BonkHttpResult<Dashboard> {
    let state = state.lock().expect("Couldn't acquire state");

    let dashboard = state
        .dashboards
        .iter()
        .find(|dashboard| dashboard.name == body.name)
        .cloned()
        .context("couldn't find dashboard")?;

    Ok(AppJson(dashboard))
}
