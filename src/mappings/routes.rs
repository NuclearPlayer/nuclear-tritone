use axum::routing::{delete, post, put};
use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/top", post(get_top))
        .route("/", put(verify))
        .route("/", delete(unverify))
}

async fn get_top() {}

async fn verify() {}

async fn unverify() {}
