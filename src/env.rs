use std::env;

pub struct Env {
    pub database_url: String,
}

impl Env {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let database_url = required("DATABASE_URL");

        Self { database_url }
    }
}

fn required(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        panic!("Missing required environment variable: {key}");
    })
}
