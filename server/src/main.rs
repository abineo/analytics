use tokio::sync::mpsc::channel;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::api::server::Server;
use crate::config::AppConfig;
use crate::database::Database;
use crate::shutdown::shutdown_signal;

mod api;
mod config;
mod database;
mod log;
mod shutdown;

/// - https://github.com/monperrus/crawler-user-agents/blob/master/crawler-user-agents.json
/// - https://cubot.net
const BOT_REGEX: &'static str = "(?<!cu)bot|crawl|spider|google|lighthouse|slurp";

include!(concat!(env!("OUT_DIR"), "/timezone-codegen.rs"));

#[tokio::main]
async fn main() {
    // dbg!(&TIMEZONES.get("Europe/Zurich"));

    log::setup();

    info!("Hello");

    let config = AppConfig::load();
    let database = Database::new(&config.database);
    let server = Server::new(config, database.clone());

    let (send, mut recv) = channel::<()>(1);
    let token = CancellationToken::new();

    database.connect(token.clone(), send.clone());
    server.start(token.clone(), send);

    shutdown_signal().await;
    token.cancel();

    let _ = recv.recv().await;

    info!("Bye");
}
