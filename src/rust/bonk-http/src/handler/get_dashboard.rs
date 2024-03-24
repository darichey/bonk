use anyhow::Context;
use axum::{debug_handler, extract::State, Json};
use bonk_dashboard::Dashboard;
use serde::Deserialize;

use crate::{AppJson, AppState, BonkHttpResult};

#[derive(Deserialize)]
pub struct GetDashboardRequest {
    name: String,
}

// TODO: it's weird that this is a POST with a name in the body. We should just give dashboards IDs and use that as a route param
#[debug_handler(state = AppState)]
pub async fn get_dashboard(
    State(state): State<AppState>,
    Json(body): Json<GetDashboardRequest>,
) -> BonkHttpResult<Dashboard> {
    let dashboards = &state
        .mutable
        .lock()
        .expect("mutable state lock poisoned")
        .dashboards;

    let dashboard = dashboards
        .iter()
        .find(|dashboard| dashboard.name == body.name)
        .cloned()
        .context("couldn't find dashboard")?;

    Ok(AppJson(dashboard))
}
