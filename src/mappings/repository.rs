use std::sync::{Arc, RwLock};

use sqlx::PgPool;

use super::Mapping;

enum Storage {
    Postgres(PgPool),
    InMemory(RwLock<Vec<Mapping>>),
}

#[derive(Clone)]
pub struct MappingRepository {
    storage: Arc<Storage>,
}

impl MappingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            storage: Arc::new(Storage::Postgres(pool)),
        }
    }

    pub fn in_memory() -> Self {
        Self {
            storage: Arc::new(Storage::InMemory(RwLock::new(Vec::new()))),
        }
    }
}
