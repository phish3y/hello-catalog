use std::time::Instant;

use axum::{
    body::Bytes,
    extract::{Path, Request, State},
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use tracing::{error, info, span, warn, Level, Span};
use uuid::Uuid;

use crate::model::{AppState, Repository};

const ZIP_TYPE: &str = "application/zip";

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
    // TODO unwrap()
    if content_type != Some(&ZIP_TYPE.parse().unwrap()) {
        warn!(
            warn.kind = "put",
            expected = "application/zip",
            ?content_type,
            "unsupported content type"
        );
        return (StatusCode::UNSUPPORTED_MEDIA_TYPE).into_response();
    }

    let uuid = Uuid::new_v4().to_string();
    match state.repo.put(&uuid, body.as_ref()).await {
        Err(err) => {
            error!(error.kind="put", error.message = %err, %uuid, "failed");
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
        Ok(()) => {}
    }

    info!(info.kind="put", elapsed = start.elapsed().as_millis(), %uuid, "success");
    (StatusCode::CREATED).into_response()
}
