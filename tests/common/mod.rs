use sqlx::PgPool;
use testcontainers::core::Mount;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::postgres::Postgres;

const SCHEMA: &str = r#"
CREATE TABLE "stream-mappings" (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    artist text NOT NULL,
    title text NOT NULL,
    source text NOT NULL,
    stream_id text NOT NULL,
    author_id text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
"#;

pub struct TestDatabase {
    pub pool: PgPool,
    _container: ContainerAsync<Postgres>,
}

pub async fn setup() -> TestDatabase {
    let container = Postgres::default()
        .with_tag("16-alpine")
        .with_mount(Mount::tmpfs_mount("/var/lib/postgresql/data"))
        .start()
        .await
        .expect("Failed to start postgres container");

    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get postgres port");

    let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

    let pool = PgPool::connect(&url)
        .await
        .expect("Failed to connect to test database");

    sqlx::query(SCHEMA)
        .execute(&pool)
        .await
        .expect("Failed to create schema");

    TestDatabase {
        pool,
        _container: container,
    }
}
