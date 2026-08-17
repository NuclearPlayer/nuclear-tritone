use tokio::net::TcpListener;

use nuclear_tritone::{app, env, state};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let env = env::Env::load();
    tracing::info!("Environment loaded");

    let state = state::AppState::from_env(&env).await;

    let app = app(state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    tracing::info!("Listening on port 3000");
    axum::serve(listener, app).await.expect("Server error");
}
