use sqlx::PgPool;

#[derive(Clone)]
pub struct MappingRepository {
    pool: PgPool,
}

impl MappingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
