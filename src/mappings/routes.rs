use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, post, put};
use axum::{Json, Router};

use std::sync::Arc;

use super::{top_stream, Mapping, TopStream, TrackKey};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/top", post(get_top))
        .route("/", put(verify))
        .route("/", delete(unverify))
}

#[derive(serde::Deserialize)]
struct TopStreamRequest {
    artist: String,
    title: String,
    source: String,
    author_id: Option<String>,
}

async fn get_top(
    State(state): State<AppState>,
    payload: Result<Json<TopStreamRequest>, JsonRejection>,
) -> Result<Json<TopStream>, StatusCode> {
    let Ok(Json(request)) = payload else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let TopStreamRequest {
        artist,
        title,
        source,
        author_id,
    } = request;

    let key = TrackKey {
        artist,
        title,
        source,
    };

    let mappings = mappings_for(&state, key).await?;

    top_stream(&mappings, author_id.as_deref())
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn mappings_for(state: &AppState, key: TrackKey) -> Result<Arc<Vec<Mapping>>, StatusCode> {
    if let Some(cached) = state.mapping_cache.get(&key) {
        return Ok(cached);
    }

    let fetched = state
        .mappings
        .find_all(&key.artist, &key.title, &key.source)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mappings = Arc::new(fetched);

    if !mappings.is_empty() {
        state.mapping_cache.insert(key, Arc::clone(&mappings));
    }

    Ok(mappings)
}

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
