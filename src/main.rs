use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
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

    let database_url = std::env::var("DATABASE_URL").ok().unwrap_or(String::from(
        "postgres://docker:docker@localhost:5432/foodfy",
    ));

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let app = Router::new()
        .nest("/v1", v1_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

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
