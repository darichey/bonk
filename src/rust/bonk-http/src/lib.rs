pub mod cli;
mod handler;
mod watch;

use std::{
    ops::Deref,
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;
use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use bonk_chat::ChatBot;
use bonk_check::WorkspaceExt as _;
use bonk_db::Db;
use bonk_parse::WorkspaceExt as _;
use bonk_workspace::Workspace;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::{
    handler::{
        chat::chat, get_dashboard::get_dashboard, get_dashboard_names::get_dashboard_names,
        get_query::get_query, get_query_names::get_query_names, get_transactions::get_transactions,
        live_reload::live_reload, query_transactions::query_transactions,
        query_transactions_for_chart::query_transactions_for_chart,
        render_query_template::render_query_template,
    },
    watch::Watcher,
};

pub fn run(cfg: &Path, host: &str, port: u16) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run_async(cfg, host, port).await })
}

async fn run_async(cfg: &Path, host: &str, port: u16) -> anyhow::Result<()> {
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
        .route("/queryNames", get(get_query_names))
        .route("/query", post(get_query))
        .route("/chat", post(chat))
        .layer(
            // TODO: real cors
            ServiceBuilder::new().layer(CorsLayer::permissive()),
        )
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

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
    workspace: Workspace,
}

impl MutableAppState {
    fn new(cfg: &Path) -> anyhow::Result<Self> {
        let workspace = Workspace::from_cfg(cfg).map_err(|err| anyhow!(err))?;
        let parsed_workspace = workspace.parse()?;
        let checked_workspace = parsed_workspace
            .check()
            .map_err(|_| anyhow!("check failed"))?;

        Ok(MutableAppState {
            db: Db::new(&checked_workspace, ":memory:").expect("Couldn't create database"),
            workspace,
        })
    }
}

struct InnerAppState {
    mutable: Arc<Mutex<MutableAppState>>,
    watcher: Watcher,
    chat_bot: ChatBot,
}

impl AppState {
    fn new(cfg: &Path) -> anyhow::Result<Self> {
        let mutable = Arc::new(Mutex::new(MutableAppState::new(cfg)?));

        let state = AppState(Arc::new(InnerAppState {
            mutable: mutable.clone(),
            watcher: Watcher::new(cfg, mutable)?,
            chat_bot: ChatBot::new(),
        }));

        Ok(state)
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
