use sqlx::postgres::PgPoolOptions;

use crate::env::Env;
use crate::mappings::MappingRepository;

#[derive(Clone)]
pub struct AppState {
    pub mappings: MappingRepository,
}

impl AppState {
    pub async fn from_env(env: &Env) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env.database_url)
            .await
            .expect("Failed to connect to database");

        Self {
            mappings: MappingRepository::new(pool),
        }
    }
}
