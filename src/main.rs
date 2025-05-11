use std::sync::Arc;

use axum::{
    routing::{get, put},
    Router,
};
use model::{AppState, S3Repo};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, span, Level};

mod handlers;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();
    let span = span!(Level::INFO, "main", context = "main");
    let _e = span.enter();

    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let state: AppState<S3Repo> = AppState {
        repo: Arc::new(S3Repo::new("".to_string()).await),
    };
    let app = Router::new()
        .route("/healthz", get(handlers::healthz))
        .route("/readyz", get(handlers::readyz))
        .route("/api/file/:id", get(handlers::get_file))
        .route("/api/file/:id", put(handlers::put_file))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    info!("API at: {}", addr);
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to create tcp listener");
    axum::serve(listener, app)
        .await
        .expect("failed to create axum service");
}
