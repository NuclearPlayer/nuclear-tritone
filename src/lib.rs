pub mod env;
pub mod mappings;
pub mod state;

use axum::Router;
use tower_http::cors::CorsLayer;

pub fn test_app() -> Router {
    let state = state::AppState {
        mappings: mappings::MappingRepository::in_memory(),
    };

    Router::new()
        .nest("/mappings", mappings::routes())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
