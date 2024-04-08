use axum::{debug_handler, extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{AppJson, AppState, BonkHttpResult};

#[derive(Deserialize)]
pub struct ChatRequest {
    prompt: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    response: String,
}

// TODO: stream response
#[debug_handler(state = AppState)]
pub async fn chat(
    State(state): State<AppState>,
    Json(body): Json<ChatRequest>,
) -> BonkHttpResult<ChatResponse> {
    let response = state.chat_bot.generate(&body.prompt).await;
    Ok(AppJson(ChatResponse { response }))
}
