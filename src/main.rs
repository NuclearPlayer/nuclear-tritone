use tokio::net::TcpListener;

use nuclear_tritone::{app, env, state};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let env = env::Env::load();
    tracing::info!("Environment loaded");

    let state = state::AppState::from_env(&env).await;

    let app = app(state);

    let address = format!("0.0.0.0:{}", env.port);
    let listener = TcpListener::bind(&address)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {address}"));

    tracing::info!("Listening on {address}");
    axum::serve(listener, app).await.expect("Server error");
}
