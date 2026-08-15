use sqlx::PgPool;

#[derive(Clone)]
pub struct VerificationRepository {
    pool: PgPool,
}

impl VerificationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
