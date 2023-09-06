use std::time::Duration;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, Pool, Postgres};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};

use crate::config::DatabaseConfig;

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub fn new(config: &DatabaseConfig) -> Self {
        let options = PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.username)
            .password(&config.password)
            .database(&config.database)
            .disable_statement_logging()
            .clone();

        debug!(
            "using database {}@{}:{}/{}",
            config.username, config.host, config.port, config.database
        );

        let timeout = Duration::from_secs(config.timeout_seconds);
        let pool = PgPoolOptions::default()
            .min_connections(config.min_connections)
            .max_connections(config.max_connections)
            .acquire_timeout(timeout)
            .connect_lazy_with(options);

        info!("database ready");

        Database { pool }
    }

    pub fn connect(self, token: CancellationToken, send: Sender<()>) {
        tokio::spawn(async move {
            let _send = send;

            info!("connecting to database; running pending migrations");
            if let Err(err) = sqlx::migrate!().run(&self.pool).await {
                error!("{err}")
            }

            token.cancelled().await;
            info!("shutdown database");

            self.pool.close().await;
            info!("database closed");
        });
    }
}
