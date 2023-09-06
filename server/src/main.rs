use tokio::sync::mpsc::channel;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::config::AppConfig;
use crate::database::Database;
use crate::server::Server;
use crate::shutdown::shutdown_signal;

mod config;
mod database;
mod log;
mod server;
mod shutdown;

/// - https://github.com/monperrus/crawler-user-agents/blob/master/crawler-user-agents.json
const BOT_REGEX: &'static str = "bot|spider|crawl|google|slurp|lighthouse";

/// - https://cubot.net
const NOT_BOT_REGEX: &'static str = "cubot";

include!(concat!(env!("OUT_DIR"), "/timezone-codegen.rs"));

#[tokio::main]
async fn main() {
    // dbg!(&TIMEZONES.get("Europe/Zurich"));
    
    log::setup();

    info!("Hello");

    let config = AppConfig::load();
    let database = Database::new(&config.database);
    let server = Server::new(config.api);

    let (send, mut recv) = channel::<()>(1);
    let token = CancellationToken::new();

    database.connect(token.clone(), send.clone());
    server.start(token.clone(), send);

    shutdown_signal().await;
    token.cancel();

    info!("waiting for tasks to finish");
    let _ = recv.recv().await;

    info!("Bye");
}
