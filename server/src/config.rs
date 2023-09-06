use std::time::Duration;

use serde::Deserialize;
use tracing::info;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    #[serde(with = "humantime_serde")]
    pub timeout: Duration,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub timeout_seconds: u64,
}

impl AppConfig {
    pub fn load() -> Self {
        #[cfg(feature = "dotenv")]
        dotenv::dotenv().ok();

        let config = config::Config::builder()
            .add_source(config::File::with_name("config/server.toml"))
            .add_source(config::Environment::default().separator("_"))
            .build()
            .expect("can build valid config")
            .try_deserialize()
            .expect("config can be deserialized");

        info!("config loaded");

        config
    }
}
