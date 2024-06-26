use std::convert::Infallible;

use axum::{
    debug_handler,
    extract::State,
    response::{sse::Event, Sse},
};
use futures::{Stream, StreamExt};
use tokio_stream::wrappers::BroadcastStream;

use crate::AppState;

#[debug_handler(state = AppState)]
pub async fn live_reload(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = BroadcastStream::new(state.watcher.events_tx.subscribe())
        // TODO: in the future, this could incldue info about what changed so we can be more conservative in our refetch
        .map(|_| Ok(axum::response::sse::Event::default().data("reload")));

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default().text("keep-alive"))
}
