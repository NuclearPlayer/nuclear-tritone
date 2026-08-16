pub mod env;
pub mod mappings;
pub mod state;

use axum::Router;
use tower_http::cors::CorsLayer;

pub struct TestApp {
    mappings: Vec<mappings::Mapping>,
}

pub fn test_app() -> TestApp {
    TestApp {
        mappings: Vec::new(),
    }
}

impl TestApp {
    pub fn with_mappings(mut self, mappings: Vec<mappings::Mapping>) -> Self {
        self.mappings = mappings;
        self
    }

    pub fn build(self) -> Router {
        let state = state::AppState {
            mappings: mappings::MappingRepository::in_memory_with(self.mappings),
        };

        Router::new()
            .nest("/mappings", mappings::routes())
            .layer(CorsLayer::permissive())
            .with_state(state)
    }
}
