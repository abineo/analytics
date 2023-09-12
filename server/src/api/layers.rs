use std::time::{Duration, Instant};

use axum::http::HeaderValue;
use axum::{http::Request, middleware::Next, response::Response};
use chrono::{DateTime, Utc};
use humantime::format_duration;

use crate::config::ApiConfig;

#[derive(Debug, Clone)]
pub struct Timestamp(pub DateTime<Utc>);

#[derive(Debug, Clone)]
pub struct Latency(pub Duration);

pub async fn response_time<B>(mut request: Request<B>, next: Next<B>) -> Response {
    let start = Instant::now();
    request.extensions_mut().insert(Timestamp(Utc::now()));

    let mut response = next.run(request).await;

    let duration = start.elapsed();
    let header = match format_duration(duration).to_string().parse() {
        Ok(header) => header,
        Err(_) => HeaderValue::from_static("?"),
    };
    response.headers_mut().insert("X-Response-Time", header);
    response.extensions_mut().insert(Latency(duration));
    response
}

pub fn cors_layer(config: &ApiConfig) -> tower_http::cors::CorsLayer {
    let scheme = match config.secure {
        true => "https",
        false => "http",
    };
    match format!("{scheme}://{}", &config.origin)
        .as_str()
        .parse::<HeaderValue>()
    {
        Ok(origin) => tower_http::cors::CorsLayer::very_permissive().allow_origin(origin),
        Err(_) => tower_http::cors::CorsLayer::very_permissive(),
    }
}
