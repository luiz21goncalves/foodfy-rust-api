use axum::Router;
use sqlx::{Pool, Postgres};

pub mod status;

pub fn app_routes() -> Router<Pool<Postgres>> {
    let status_routes = status::status_routes();

    Router::new().nest("/v1", status_routes)
}
