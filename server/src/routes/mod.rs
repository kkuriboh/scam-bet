use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::database;
use handlers::signup;

mod handlers;

pub struct InnerState {
    redis: database::cache::Pool,
    scylla: database::main_db::Pool,
}

type AppState = Arc<InnerState>;

pub async fn make_routes() -> anyhow::Result<Router> {
    Ok(Router::new()
        .route("/api/ping", get(|| async { "pong" }))
        .route("/api/signup", post(signup))
        .with_state(Arc::new(InnerState {
            redis: database::cache::connect(env!("REDIS_URL")).await?,
            scylla: database::main_db::connect(env!("SCYLLA_URL")).await?,
        })))
}
