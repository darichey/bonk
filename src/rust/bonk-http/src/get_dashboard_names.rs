use std::sync::{Arc, Mutex};

use rouille::{Request, Response};

use crate::{try_or_400, State};

pub(crate) fn get_dashboard_names(request: &Request, state: Arc<Mutex<State>>) -> Response {
    let body = try_or_400!(handle(request, state));
    Response::json(&body)
}

fn handle(_request: &Request, state: Arc<Mutex<State>>) -> anyhow::Result<Vec<String>> {
    let state = state.lock().expect("Couldn't acquire state");

    Ok(state
        .dashboards
        .iter()
        .map(|dashboard| dashboard.name.clone())
        .collect())
}
