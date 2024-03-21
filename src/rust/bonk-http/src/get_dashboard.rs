use std::sync::{Arc, Mutex};

use anyhow::Context;
use bonk_dashboard::Dashboard;
use rouille::{Request, Response};
use serde::Deserialize;

use crate::{try_or_400, State};

#[derive(Deserialize)]
struct GetDashboardRequest {
    name: String,
}

// TODO: it's weird that this is a POST with a name in the body. We should just give dashboards IDs and use that as a route param
pub(crate) fn get_dashboard(request: &Request, state: Arc<Mutex<State>>) -> Response {
    let body = try_or_400!(handle(request, state));
    Response::json(&body)
}

fn handle(request: &Request, state: Arc<Mutex<State>>) -> anyhow::Result<Dashboard> {
    let state = state.lock().expect("Couldn't acquire state");

    let body: GetDashboardRequest = rouille::input::json_input(request)?;

    state
        .dashboards
        .iter()
        .find(|dashboard| dashboard.name == body.name)
        .cloned()
        .context("couldn't find dashboard")
}
