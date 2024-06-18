use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::{net::TcpListener, signal};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod routes;

async fn app() -> Router {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("cannot find DATABASE_URL env");

    let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    routes::app_routes()
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
        ))
        .with_state(pool)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = std::env::var("PORT").expect("cannot find PORT env");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    tracing::debug!(
        "http server listening on: {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app().await)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}
