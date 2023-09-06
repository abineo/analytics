use std::env;

use convert_case::{Case, Casing};
use tracing::Level;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{registry, Layer};

pub fn setup() {
    registry()
        .with(layer().with_filter(targets_from_env()))
        .init()
}

fn targets_from_env() -> Targets {
    let log_level = env::var("LOG_LEVEL").unwrap_or_default();
    match log_level.to_lowercase().as_str() {
        "debug" => debug(),
        "trace" => trace(),
        _ => default(),
    }
}

fn default() -> Targets {
    Targets::new()
        .with_default(Level::WARN)
        .with_target(env!("CARGO_PKG_NAME").to_case(Case::Snake), Level::DEBUG)
}

fn debug() -> Targets {
    Targets::new().with_default(Level::DEBUG)
}

fn trace() -> Targets {
    Targets::new().with_default(Level::TRACE)
}
