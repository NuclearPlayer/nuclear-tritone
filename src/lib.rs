pub mod env;
pub mod mappings;
pub mod state;

use axum::Router;
use tower_http::cors::CorsLayer;

use state::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .nest("/mappings", mappings::routes())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
