use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

use crate::controllers;

pub fn status_routes() -> Router<Pool<Postgres>> {
    let routes = Router::new().route("/", get(controllers::status::api_status));

    Router::new().nest("/status", routes)
}
