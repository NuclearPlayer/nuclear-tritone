use sqlx::postgres::PgPoolOptions;

use crate::env::Env;
use crate::verifications::VerificationRepository;

#[derive(Clone)]
pub struct AppState {
    pub verifications: VerificationRepository,
}

impl AppState {
    pub async fn from_env(env: &Env) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env.database_url)
            .await
            .expect("Failed to connect to database");

        Self {
            verifications: VerificationRepository::new(pool),
        }
    }
}
