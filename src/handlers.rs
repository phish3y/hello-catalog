use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::{span, Level};

pub async fn healthz() -> impl IntoResponse {
  (StatusCode::OK, Json({}))
}

pub async fn readyz() -> impl IntoResponse {
  (StatusCode::OK, Json({}))
}

pub async fn put_file() -> impl IntoResponse {
  let span = span!(Level::INFO, "put_file", context = "put_file");
  let _e = span.enter();
  
  
}