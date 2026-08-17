use axum_test::TestServer;
use sqlx::PgPool;
use testcontainers::core::Mount;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::postgres::Postgres;

use nuclear_tritone::app;
use nuclear_tritone::mappings::{Mapping, MappingRepository};
use nuclear_tritone::state::AppState;

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

pub struct TestApp {
    pub server: TestServer,
    mappings: MappingRepository,
    _container: ContainerAsync<Postgres>,
}

pub async fn setup() -> TestApp {
    let (pool, container) = setup_database().await;
    let mappings = MappingRepository::new(pool);

    let state = AppState::new(mappings.clone());

    let server = TestServer::new(app(state));

    TestApp {
        server,
        mappings,
        _container: container,
    }
}

impl TestApp {
    pub async fn init_mappings(&self, mappings: Vec<Mapping>) {
        for mapping in &mappings {
            self.mappings
                .insert(mapping)
                .await
                .expect("Failed to insert mapping");
        }
    }
}

async fn setup_database() -> (PgPool, ContainerAsync<Postgres>) {
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

    (pool, container)
}
