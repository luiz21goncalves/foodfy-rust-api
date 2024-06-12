use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let status_routes = Router::new().route("/", get(controllers::status::api_status));

    let v1_routes = Router::new().nest("/status", status_routes);

    let app = Router::new()
        .nest("/v1", v1_routes)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").ok().unwrap_or(String::from("3333"));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::debug!(
        "http server listening on: {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap()
}
