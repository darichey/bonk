pub mod cli;
mod handler;
mod watch;

use std::{
    ops::Deref,
    path::Path,
    sync::{Arc, Mutex},
};

use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use bonk_check::WorkspaceExt as _;
use bonk_dashboard::Dashboard;
use bonk_db::Db;
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use serde::Serialize;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::{
    handler::{
        get_dashboard::get_dashboard, get_dashboard_names::get_dashboard_names,
        get_transactions::get_transactions, live_reload::live_reload,
        query_transactions::query_transactions,
        query_transactions_for_chart::query_transactions_for_chart,
        render_query_template::render_query_template,
    },
    watch::Watcher,
};

pub fn run(cfg: &Path) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run_async(cfg).await })
}

async fn run_async(cfg: &Path) -> anyhow::Result<()> {
    let state = AppState::new(cfg)?;

    let app = Router::new()
        .route("/transactions", get(get_transactions))
        .route("/queryTransactions", post(query_transactions))
        .route("/dashboard", post(get_dashboard))
        .route("/dashboardNames", get(get_dashboard_names))
        .route(
            "/queryTransactionsForChart",
            post(query_transactions_for_chart),
        )
        .route("/renderQueryTemplate", post(render_query_template))
        .route("/liveReload", get(live_reload))
        .layer(
            // TODO: real cors
            ServiceBuilder::new().layer(CorsLayer::permissive()),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:8080").await?;

    println!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState(Arc<InnerAppState>);

impl Deref for AppState {
    type Target = InnerAppState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MutableAppState {
    db: Db,
    dashboards: Vec<Dashboard>,
}

impl MutableAppState {
    fn new(cfg: &Path) -> Self {
        let workspace = Workspace::from_cfg(cfg).expect("Couldn't read cfg");
        let parsed_workspace = workspace.parse().unwrap();
        let checked_workspace = parsed_workspace.check().unwrap();

        MutableAppState {
            db: Db::new(&checked_workspace, ":memory:").expect("Couldn't create database"),
            dashboards: workspace.cfg.dashboards,
        }
    }
}

struct InnerAppState {
    mutable: Arc<Mutex<MutableAppState>>,
    watcher: Watcher,
}

impl AppState {
    fn new(cfg: &Path) -> anyhow::Result<Self> {
        let mutable = Arc::new(Mutex::new(MutableAppState::new(cfg)));

        let state = AppState(Arc::new(InnerAppState {
            mutable: mutable.clone(),
            watcher: Watcher::new(cfg, mutable)?,
        }));

        Ok(state)
    }
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

type BonkHttpResult<T> = Result<AppJson<T>, AppError>;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
