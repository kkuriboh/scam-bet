use axum::{extract::State, response::IntoResponse};

use super::AppState;

pub async fn signup(State(state): State<AppState>) -> impl IntoResponse {
    let _ = state.redis;
    let _ = state.scylla;
}
