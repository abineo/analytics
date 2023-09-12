use axum::extract::{Path, State};
use axum::headers::UserAgent;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router, TypedHeader};
use serde::Deserialize;
use serde_json::Value;

use crate::api::server::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(|| async { StatusCode::OK }))
        .nest(
            "/v1/:project_id",
            Router::new()
                .route("/visit", post(visit))
                .route("/exit", post(exit))
                .route("/event", post(event)),
        )
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Visitor {
    timezone: String,
    language: String,
    screen: (i32, i32),
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Page {
    host: String,
    path: String,
    search: Option<String>,
    referrer: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Visit {
    session: String,
    visitor: Visitor,
    page: Page,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Exit {
    session: String,
    visitor: Visitor,
    page: Page,
    duration: i64,
    distance: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Event {
    session: String,
    visitor: Visitor,
    page: Page,
    name: String,
    attr: Value,
}

async fn visit(
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Path(project_id): Path<String>,
    Json(visit): Json<Visit>,
) -> impl IntoResponse {
    dbg!(project_id, user_agent, visit);
    ""
}

async fn exit(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(exit): Json<Exit>,
) -> impl IntoResponse {
    dbg!(project_id, exit);
    ""
}

async fn event(
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Path(project_id): Path<String>,
    Json(event): Json<Event>,
) -> impl IntoResponse {
    dbg!(project_id, user_agent, event);
    ""
}
