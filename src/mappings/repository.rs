use sqlx::PgPool;

use super::Mapping;

#[derive(Clone)]
pub struct MappingRepository {
    pool: PgPool,
}

impl MappingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, mapping: &Mapping) -> sqlx::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO "stream-mappings" (artist, title, source, stream_id, author_id)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(&mapping.artist)
        .bind(&mapping.title)
        .bind(&mapping.source)
        .bind(&mapping.stream_id)
        .bind(&mapping.author_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_all(
        &self,
        artist: &str,
        title: &str,
        source: &str,
    ) -> sqlx::Result<Vec<Mapping>> {
        sqlx::query_as(
            r#"
            SELECT artist, title, source, stream_id, author_id
            FROM "stream-mappings"
            WHERE artist = $1 AND title = $2 AND source = $3
            "#,
        )
        .bind(artist)
        .bind(title)
        .bind(source)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete(
        &self,
        artist: &str,
        title: &str,
        source: &str,
        author_id: &str,
    ) -> sqlx::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM "stream-mappings"
            WHERE artist = $1 AND title = $2 AND source = $3 AND author_id = $4
            "#,
        )
        .bind(artist)
        .bind(title)
        .bind(source)
        .bind(author_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
