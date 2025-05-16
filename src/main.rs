use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, put},
    Router,
};
use model::{AppState, KafkaNotifier, S3Repo};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, span, Level, Span};

mod handlers;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();
    let span: Span = span!(Level::INFO, "main", context = "main");
    let _e = span.enter();

    let port: String = std::env::var("PORT").unwrap_or("3000".to_string());
    let bucket: String = std::env::var("BUCKET").expect("failed to parse env var BUCKET: required");
    let body_limit: usize = std::env::var("BODY_LIMIT_MB")
        .unwrap_or("20".to_string())
        .parse()
        .expect("failed to parse env var BODY_LIMIT_MB: must be usize");
    let kafka_brokers: String =
        std::env::var("KAFKA_BROKERS").expect("failed to parse env var KAFKA_BROKERS: required");

    info!(info.kind = "main", %bucket, "bucket");
    info!(info.kind = "main", %body_limit, "body limit");
    info!(info.kind = "main", %kafka_brokers, "kafka brokers");

    let state: AppState<S3Repo, KafkaNotifier> = AppState {
        repo: Arc::new(S3Repo::new(bucket).await),
        notifier: Arc::new(
            KafkaNotifier::new(&kafka_brokers)
                .await
                .expect("failed to create kafka notifier"),
        ),
    };
    let app = Router::new()
        .route("/healthz", get(handlers::healthz))
        .route("/readyz", get(handlers::readyz))
        .route("/api/file/{id}", get(handlers::get_file))
        .route(
            "/api/file",
            put(handlers::put_file).layer(DefaultBodyLimit::max(body_limit * 1024 * 1024)),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: String = format!("0.0.0.0:{}", port);
    info!(info.kind = "main", %addr, "api started");
    let listener: TcpListener = TcpListener::bind(addr)
        .await
        .expect("failed to create tcp listener");
    axum::serve(listener, app)
        .await
        .expect("failed to create axum service");
}
