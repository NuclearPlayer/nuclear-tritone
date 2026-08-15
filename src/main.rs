mod env;
mod state;
mod mappings;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let env = env::Env::load();
    tracing::info!("Environment loaded");

    let state = state::AppState::from_env(&env).await;

    let app = Router::new()
        .nest("/verifications", mappings::routes())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("Listening on port 3000");
    axum::serve(listener, app).await.expect("Server error");
}
