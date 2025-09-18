use dotenvy::dotenv;
use std::env;

pub struct AppConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = env::var("APP_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);

        Self { database_url, host, port }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
