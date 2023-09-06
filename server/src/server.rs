use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::config::ApiConfig;

#[derive(Debug, Clone)]
pub struct Visitor {
    id: i64,
    tz: String,
    lang: String,
}

#[derive(Debug, Clone)]
pub struct Server {
    config: ApiConfig,
}

impl Server {
    pub fn new(config: ApiConfig) -> Self {
        info!("server ready");
        Server { config }
    }

    pub fn start(self, token: CancellationToken, send: Sender<()>) {
        tokio::spawn(async move {
            let _send = send;

            info!("starting server");

            // TODO: start server
            // https://docs.rs/warp/latest/warp/struct.Server.html#method.bind_with_graceful_shutdown
            // https://github.com/seanmonstar/warp/blob/master/examples/tracing.rs

            token.cancelled().await;
            info!("shutdown server");

            // TODO: stop server

            info!("server closed");
        });
    }
}
