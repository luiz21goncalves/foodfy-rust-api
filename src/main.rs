use axum::{routing::get, Json, Router};
use serde_json::json;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route(
            "/",
            get(|| async { Json(json!({"message":"Hello World"})) }),
        )
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    tracing::debug!(
        "http server listening on: {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap()
}
