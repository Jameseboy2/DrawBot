pub mod app_state;
pub mod routes;

use crate::app_state::{AppState, BotStatus};
use axum::Router;
use axum::extract::{Request, State};
use axum::routing::any;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::fmt::format::PrettyFields;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{Layer, filter};
use uuid::Uuid;

#[allow(clippy::unwrap_used)]
#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::daily("logs", "server.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(filter::LevelFilter::DEBUG)) // Logs to stdout
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_file)
                .with_ansi(false)
                .fmt_fields(PrettyFields::new())
                .with_filter(filter::LevelFilter::INFO),
        ) // Logs to file
        .init();

    let shared_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/ws", any(routes::ws::handle))
        .with_state(shared_state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request| {
                let request_id = Uuid::new_v4().to_string();

                tracing::info_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri()
                )
            }),
        );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
