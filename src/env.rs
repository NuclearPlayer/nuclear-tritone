use std::env;

pub struct Env {
    pub database_url: String,
    pub port: u16,
}

impl Env {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let database_url = required("DATABASE_URL");
        let port = env::var("PORT").map_or(8080, |value| value.parse().expect("Invalid PORT"));

        Self { database_url, port }
    }
}

fn required(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        panic!("Missing required environment variable: {key}");
    })
}
