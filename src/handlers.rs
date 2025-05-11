use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::{span, Level, Span};

use crate::model::{AppState, Repository};

pub async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, Json({}))
}

pub async fn readyz() -> impl IntoResponse {
    (StatusCode::OK, Json({}))
}

pub async fn get_file<R: Repository>(
    State(state): State<AppState<R>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let span: Span = span!(Level::INFO, "get_file", context = "get_file");
    let _e = span.enter();
}

pub async fn put_file<R: Repository>(
    State(state): State<AppState<R>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let span: Span = span!(Level::INFO, "put_file", context = "put_file");
    let _e = span.enter();
}
