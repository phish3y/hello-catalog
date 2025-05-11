use std::time::Instant;

use axum::{
    body::Bytes, extract::{Path, Request, State}, http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode}, response::IntoResponse, Json
};
use tracing::{info, span, Level, Span};

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
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let start = Instant::now();

    let span: Span = span!(Level::INFO, "put_file", context = "put_file");
    let _e = span.enter();

    let content_type: Option<&HeaderValue> = headers.get(CONTENT_TYPE);
    if content_type != Some(&"application/zip".parse().unwrap()) { // TODO 
        return (StatusCode::UNSUPPORTED_MEDIA_TYPE).into_response();
    }

    // TODO

    info!("elapsed: {}ms", start.elapsed().as_millis());
    (StatusCode::CREATED).into_response()
}
