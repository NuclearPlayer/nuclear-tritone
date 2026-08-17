use std::sync::Arc;
use std::time::Duration;

use moka::sync::Cache;
use sqlx::postgres::PgPoolOptions;

use crate::env::Env;
use crate::mappings::{Mapping, MappingRepository, TrackKey};

pub type MappingCache = Cache<TrackKey, Arc<Vec<Mapping>>>;

#[derive(Clone)]
pub struct AppState {
    pub mappings: MappingRepository,
    pub mapping_cache: MappingCache,
}

impl AppState {
    pub fn new(mappings: MappingRepository) -> Self {
        let mapping_cache = Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .max_capacity(10_000)
            .build();

        Self {
            mappings,
            mapping_cache,
        }
    }

    pub async fn from_env(env: &Env) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env.database_url)
            .await
            .expect("Failed to connect to database");

        Self::new(MappingRepository::new(pool))
    }
}
