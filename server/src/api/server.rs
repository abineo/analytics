use std::net::SocketAddr;

use axum::routing::IntoMakeService;
use axum::{middleware, Router, ServiceExt};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use tower::{Layer, ServiceBuilder};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::DecompressionLayer;
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::api::layers::{cors_layer, response_time};
use crate::api::routes::routes;
use crate::config::AppConfig;
use crate::database::Database;

#[derive(Debug, Clone)]
pub struct Server {
    socket: SocketAddr,
    make_service: IntoMakeService<NormalizePath<Router>>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub database: Database,
    pub config: AppConfig,
}

impl Server {
    pub fn new(config: AppConfig, database: Database) -> Self {
        let socket: SocketAddr = SocketAddr::new(
            config.api.host.parse().expect("configured valid host"),
            config.api.port,
        );

        let layers = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(middleware::from_fn(response_time))
            .layer(CatchPanicLayer::new())
            .layer(DecompressionLayer::new())
            .layer(CompressionLayer::new())
            .layer(cors_layer(&config.api));

        let app_state = AppState { database, config };

        let router = Router::new()
            .nest("/api", routes())
            .layer(layers)
            .with_state(app_state);

        let make_service = NormalizePathLayer::trim_trailing_slash()
            .layer(router)
            .into_make_service();

        info!("server ready");
        Server {
            socket,
            make_service,
        }
    }

    pub fn start(self, token: CancellationToken, send: Sender<()>) {
        tokio::spawn(async move {
            let _send = send;

            info!("starting server at http://{}", &self.socket);

            axum::Server::bind(&self.socket)
                .serve(self.make_service)
                .with_graceful_shutdown(signal(token))
                .await
                .expect("can start server");

            info!("server closed");
        });
    }
}

async fn signal(token: CancellationToken) {
    token.cancelled().await;
    info!("shutdown server");
}
