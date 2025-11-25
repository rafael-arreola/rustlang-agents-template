use crate::{
    api::request::{ChatRequest, ChatResponse},
    state::AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use rig::completion::Prompt;
use std::sync::Arc;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    let response = state.orchestrator.agent.prompt(&payload.prompt).await;

    match response {
        Ok(text) => (StatusCode::OK, Json(ChatResponse { response: text })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response(),
    }
}
