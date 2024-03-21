pub mod cli;
mod get_dashboard;
mod get_dashboard_names;
mod get_transactions;
mod query_transactions;
mod query_transactions_for_chart;
mod render_query_template;

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use bonk_check::WorkspaceExt as _;
use bonk_dashboard::Dashboard;
use bonk_db::Db;
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use rouille::{router, Response, Server};
use serde::Serialize;

use crate::{
    get_dashboard::get_dashboard, get_dashboard_names::get_dashboard_names,
    get_transactions::get_transactions, query_transactions::query_transactions,
    query_transactions_for_chart::query_transactions_for_chart,
    render_query_template::render_query_template,
};

pub fn run(cfg: &Path) -> anyhow::Result<()> {
    let state = {
        let workspace = Workspace::from_cfg(cfg).expect("Couldn't read cfg");
        let parsed_workspace = workspace.parse().unwrap();
        let checked_workspace = parsed_workspace.check().unwrap();

        let state = State {
            db: Db::new(&checked_workspace, ":memory:").expect("Couldn't create database"),
            dashboards: workspace.cfg.dashboards,
        };

        Arc::new(Mutex::new(state))
    };

    let server = Server::new("localhost:8080", move |request| {
        if request.method() == "OPTIONS" {
            // TOOD: real cors
            return Response::empty_204()
                .with_additional_header("Access-Control-Allow-Origin", "*")
                .with_additional_header("Access-Control-Allow-Methods", "*")
                .with_additional_header("Access-Control-Allow-Headers", "*");
        }

        let response = router!(request,
            (GET) (/transactions) => { get_transactions(request, state.clone()) },
            (POST) (/queryTransactions) => { query_transactions(request, state.clone()) },
            (POST) (/dashboard) => { get_dashboard(request, state.clone()) },
            (GET) (/dashboardNames) => { get_dashboard_names(request, state.clone()) },
            (POST) (/queryTransactionsForChart) => { query_transactions_for_chart(request, state.clone()) },
            (POST) (/renderQueryTemplate) => { render_query_template(request, state.clone() )},
            _ => Response::empty_404(),
        );

        // TODO: real cors
        response.with_additional_header("Access-Control-Allow-Origin", "*")
    })
    .unwrap();

    println!("Listening on port {}", server.server_addr().port());

    server.run();

    Ok(())
}

struct State {
    db: Db,
    dashboards: Vec<Dashboard>,
}

/// A wrapper for seralizing sqlite Values
pub struct SqlValue(sqlite::Value);

impl Serialize for SqlValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            sqlite::Value::Binary(b) => serializer.serialize_bytes(b),
            sqlite::Value::Float(f) => serializer.serialize_f64(*f),
            sqlite::Value::Integer(i) => serializer.serialize_i64(*i),
            sqlite::Value::String(s) => serializer.serialize_str(s),
            sqlite::Value::Null => serializer.serialize_unit(),
        }
    }
}

// we use this instead of rouille::try_or_400 so we can use anyhow::Result and control the error response
#[macro_export]
macro_rules! try_or_400 {
    ($result:expr) => {
        match $result {
            Ok(r) => r,
            Err(err) => {
                let json = err.to_string();
                return rouille::Response::json(&json).with_status_code(400);
            }
        }
    };
}
