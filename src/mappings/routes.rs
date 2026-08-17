use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, post, put};
use axum::{Json, Router};

use super::Mapping;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/top", post(get_top))
        .route("/", put(verify))
        .route("/", delete(unverify))
}

async fn get_top() {}

async fn verify(
    State(state): State<AppState>,
    payload: Result<Json<Mapping>, JsonRejection>,
) -> StatusCode {
    let Ok(Json(mapping)) = payload else {
        return StatusCode::BAD_REQUEST;
    };

    match state.mappings.insert(&mapping).await {
        Ok(()) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(serde::Deserialize)]
struct UnverifyRequest {
    artist: String,
    title: String,
    source: String,
    author_id: String,
}

async fn unverify(
    State(state): State<AppState>,
    payload: Result<Json<UnverifyRequest>, JsonRejection>,
) -> StatusCode {
    let Ok(Json(request)) = payload else {
        return StatusCode::BAD_REQUEST;
    };

    let result = state
        .mappings
        .delete(
            &request.artist,
            &request.title,
            &request.source,
            &request.author_id,
        )
        .await;

    match result {
        Ok(()) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
